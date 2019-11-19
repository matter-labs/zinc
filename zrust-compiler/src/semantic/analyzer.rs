//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

use zrust_bytecode::Instruction;

use crate::error::Error as CompilerError;
use crate::lexical::Literal as InnerLiteral;
use crate::lexical::Location;
use crate::semantic::inference;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error;
use crate::semantic::Integer;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::Value;
use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::Statement;
use crate::syntax::Type;
use crate::syntax::TypeVariant;
use crate::CircuitProgram;

pub struct Analyzer {
    scopes: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    instructions: Vec<Box<dyn Instruction>>,
    stack_height: usize,
}

#[derive(Debug, Clone)]
enum StackElement {
    Operand(ExpressionOperand),
    Element(Element),
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;
    const VECTOR_INSTRUCTION_INITIAL_CAPACITY: usize = 1024;

    pub fn new(scope: Scope) -> Self {
        let mut scopes = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
        scopes.push(Rc::new(RefCell::new(scope)));

        let operands = Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY);

        let instructions = Vec::with_capacity(Self::VECTOR_INSTRUCTION_INITIAL_CAPACITY);

        Self {
            scopes,
            operands,
            instructions,
            stack_height: 0,
        }
    }

    pub fn compile(
        mut self,
        program: CircuitProgram,
    ) -> Result<Vec<Box<dyn Instruction>>, CompilerError> {
        for statement in program.statements.into_iter() {
            self.execute_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        Ok(self.instructions)
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Empty => {}
            Statement::Let(r#let) => {
                let location = r#let.location;
                self.evaluate_expression(r#let.expression)?;
                let type_variant = if let Some(r#type) = r#let.r#type {
                    let let_type_variant = match r#type.variant {
                        TypeVariant::Alias { identifier } => {
                            let location = r#type.location;
                            self.scope()
                                .borrow()
                                .resolve_type(&identifier)
                                .map_err(|error| Error::Scope(location, error))?
                        }
                        type_variant => type_variant,
                    };

                    let operand_1 = self.get_unary_operand(true)?;
                    let (is_signed, bitlength) = operand_1
                        .cast(
                            Element::Type(let_type_variant.clone()),
                            self.scope().borrow().deref(),
                        )
                        .map_err(|error| Error::Element(location, error))?;
                    self.push_instruction(Box::new(zrust_bytecode::Cast::new(
                        is_signed,
                        bitlength as u8,
                    )));
                    let_type_variant
                } else {
                    let operand_1 = self.get_unary_operand(true)?;
                    let type_variant = operand_1
                        .type_variant(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(location, error))?;
                    type_variant
                };

                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        Place::new(r#let.identifier.name),
                        Value::new(type_variant),
                        r#let.is_mutable,
                        self.stack_height - 1,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Loop(r#loop) => {
                let location = r#loop.location;
                let range_bitlength =
                    inference::enough_bitlength(&[&r#loop.range_start, &r#loop.range_end])
                        .map_err(|error| Error::Inference(location, error))?;
                let value = Value::new(TypeVariant::new_integer_unsigned(range_bitlength));

                let range_start: usize = r#loop.range_start.into();
                let range_end: usize = r#loop.range_end.into();
                let iterations_number = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if r#loop.is_range_inclusive { 1 } else { 0 };

                self.push_instruction(Box::new(zrust_bytecode::LoopBegin::new(
                    iterations_number,
                    1,
                )));
                self.push_instruction(Box::new(value.to_push()));
                self.push_scope();
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        Place::new(r#loop.index_identifier.name),
                        value,
                        false,
                        self.stack_height,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                self.stack_height += 1;
                self.evaluate_block_expression(r#loop.block)?;
                self.pop_scope();
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(self.stack_height - 1)));
                self.push_instruction(Box::new(Integer::increment(range_bitlength).to_push()));
                self.push_instruction(Box::new(zrust_bytecode::Add));
                self.push_instruction(Box::new(zrust_bytecode::LoopEnd));
            }
            Statement::Type(r#type) => {
                let location = r#type.location;
                self.scope()
                    .borrow_mut()
                    .declare_type(r#type.identifier.name, r#type.r#type.variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Struct(r#struct) => {
                let location = r#struct.location;
                let type_variant = TypeVariant::new_structure(
                    r#struct
                        .fields
                        .into_iter()
                        .map(|field| (field.identifier.name, field.r#type.variant))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#struct.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Enum(r#enum) => {
                let location = r#enum.location;
                let type_variant = TypeVariant::new_enumeration(
                    r#enum
                        .variants
                        .into_iter()
                        .map(|variant| (variant.identifier.name, variant.literal))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#enum.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Fn(r#fn) => {
                let location = r#fn.location;
                let type_variant = TypeVariant::new_function(
                    r#fn.arguments
                        .into_iter()
                        .map(|field| (field.identifier.name, field.r#type.variant))
                        .collect(),
                    r#fn.return_type.variant,
                    r#fn.body,
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#fn.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Mod(_mod) => {}
            Statement::Use(_use) => {}
            Statement::Expression(expression) => {
                self.evaluate_expression(expression)?;
            }
        }

        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<(), Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::Operand(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, true)?;
                    let place = operand_1
                        .assign(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.scope()
                        .borrow_mut()
                        .update_variable(place, self.stack_height - 1)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.push_operand(StackElement::Element(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {}
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {}
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Or));
                    self.stack_height -= 1;

                    let result = operand_1
                        .or(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Xor));
                    self.stack_height -= 1;

                    let result = operand_1
                        .xor(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::And));
                    self.stack_height -= 1;

                    let result = operand_1
                        .and(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Eq));
                    self.stack_height -= 1;

                    let result = operand_1
                        .equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Ne));
                    self.stack_height -= 1;

                    let result = operand_1
                        .not_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Ge));
                    self.stack_height -= 1;

                    let result = operand_1
                        .greater_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Le));
                    self.stack_height -= 1;

                    let result = operand_1
                        .lesser_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Gt));
                    self.stack_height -= 1;

                    let result = operand_1
                        .greater(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Lt));
                    self.stack_height -= 1;

                    let result = operand_1
                        .lesser(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Add));
                    self.stack_height -= 1;

                    let result = operand_1
                        .add(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Sub));
                    self.stack_height -= 1;

                    let result = operand_1
                        .subtract(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Mul));
                    self.stack_height -= 1;

                    let result = operand_1
                        .multiply(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Div));
                    self.stack_height -= 1;

                    let result = operand_1
                        .divide(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Rem));
                    self.stack_height -= 1;

                    let result = operand_1
                        .modulo(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, false)?;
                    let (is_signed, bitlength) = operand_1
                        .cast(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_instruction(Box::new(zrust_bytecode::Cast::new(
                        is_signed,
                        bitlength as u8,
                    )));
                    self.push_operand(StackElement::Element(Element::Value(Value::new(
                        TypeVariant::new_integer(is_signed, bitlength),
                    ))));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand(true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Neg));

                    let result = operand_1
                        .negate(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand(true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Not));

                    let result = operand_1
                        .not(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {}
                ExpressionObject::Operator(ExpressionOperator::Field) => {}
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, false)?;

                    let type_variant = match operand_1 {
                        Element::Type(type_variant) => type_variant,
                        element => return Err(Error::CallingNotCallable(element.to_string())),
                    };
                    //                    let arguments = match operand_2 {
                    //                        Element::Value(Value::List(arguments)) => arguments,
                    //                        _ => panic!("Always is an argument list"),
                    //                    };
                    //
                    //                    let value = self.evaluate_function_call(type_variant, arguments)?;
                    //                    self.push_operand(Element::Value(value));
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {}
            }
        }

        let element = self.evaluate_operand(true)?;
        self.push_operand(StackElement::Element(element));

        Ok(())
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Result<(), Error> {
        let location = literal.location;

        let value = match literal.data {
            InnerLiteral::Boolean(literal) => Value::from(literal),
            InnerLiteral::Integer(literal) => Value::try_from(literal)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
            InnerLiteral::String { .. } => panic!("String literals cannot be used in expressions"),
        };
        self.push_instruction(Box::new(value.to_push()));
        self.stack_height += 1;
        self.push_operand(StackElement::Element(Element::Value(value)));

        Ok(())
    }

    fn evaluate_identifier(
        &mut self,
        identifier: Identifier,
        is_for_stack: bool,
    ) -> Result<(), Error> {
        let location = identifier.location;
        let place = Place::new(identifier.name);
        let address = self
            .scope()
            .borrow()
            .get_variable_address(&place)
            .map_err(|error| Error::Scope(location, error))?;
        let value = self
            .scope()
            .borrow()
            .get_variable_value(&place)
            .map_err(|error| Error::Scope(location, error))?;

        if is_for_stack {
            self.push_instruction(Box::new(zrust_bytecode::Copy::new(address)));
            self.stack_height += 1;
            self.push_operand(StackElement::Element(Element::Value(value)));
        } else {
            self.push_operand(StackElement::Element(Element::Place(place)));
        }

        Ok(())
    }

    fn evaluate_type(&mut self, r#type: Type) -> Result<(), Error> {
        self.push_operand(StackElement::Element(Element::Type(r#type.variant)));

        Ok(())
    }

    fn evaluate_block_expression(&mut self, block: BlockExpression) -> Result<(), Error> {
        for statement in block.statements.into_iter() {
            self.execute_statement(statement)?;
        }
        if let Some(expression) = block.expression {
            self.evaluate_expression(*expression)?;
        } else {
            self.push_operand(StackElement::Element(Element::Value(Value::Unit)));
        }

        Ok(())
    }

    fn evaluate_conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<(), Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        self.evaluate_expression(*conditional.condition)?;
        let condition_address = self.stack_height - 1;
        match self
            .get_unary_operand(true)?
            .type_variant(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?
        {
            TypeVariant::Boolean => {}
            type_variant => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    condition_location,
                    type_variant,
                ))
            }
        }

        self.push_scope();
        self.evaluate_block_expression(conditional.main_block)?;
        let (main_result, main_result_address) =
            (self.get_unary_operand(true)?, self.stack_height - 1);
        let main_assignments = self.scope().borrow().get_assignments();
        self.pop_scope();

        let mut else_assignments = HashMap::new();
        let (else_result, else_result_address) = if let Some(else_if) = conditional.else_if {
            self.evaluate_conditional_expression(*else_if)?;
            (self.get_unary_operand(true)?, self.stack_height - 1)
        } else if let Some(else_block) = conditional.else_block {
            self.push_scope();
            self.evaluate_block_expression(else_block)?;
            else_assignments = self.scope().borrow().get_assignments();
            self.pop_scope();
            (self.get_unary_operand(true)?, self.stack_height - 1)
        } else {
            self.push_operand(StackElement::Element(Element::Value(Value::Unit)));
            (Element::Value(Value::Unit), self.stack_height)
        };

        let main_type = main_result
            .type_variant(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?;
        let else_type = else_result
            .type_variant(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?;
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypeMismatch(
                location, main_type, else_type,
            ));
        }

        match main_type {
            TypeVariant::Unit => {}
            _ => {
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(main_result_address)));
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(else_result_address)));
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(condition_address)));
                self.push_instruction(Box::new(zrust_bytecode::ConditionalSelect));
                self.stack_height += 1;
                self.push_operand(StackElement::Element(main_result));
            }
        }

        let mut assignments = HashMap::<Place, usize>::new();
        let mut conditional_assignments = HashMap::<Place, (usize, usize)>::new();
        for (place, address_1) in main_assignments.into_iter() {
            if let Some(address_2) = else_assignments.get(&place).copied() {
                conditional_assignments.insert(place, (address_1, address_2));
            } else {
                assignments.insert(place, address_1);
            }
        }
        for (place, address) in else_assignments.into_iter() {
            if !conditional_assignments.contains_key(&place) {
                assignments.insert(place, address);
            }
        }
        self.scope().borrow_mut().add_assignments(assignments);
        for (place, (address_1, address_2)) in conditional_assignments.into_iter() {
            self.push_instruction(Box::new(zrust_bytecode::Copy::new(address_2)));
            self.push_instruction(Box::new(zrust_bytecode::Copy::new(address_1)));
            self.push_instruction(Box::new(zrust_bytecode::Copy::new(condition_address)));
            self.push_instruction(Box::new(zrust_bytecode::ConditionalSelect));
            self.scope()
                .borrow_mut()
                .update_variable(place, self.stack_height)
                .map_err(|error| Error::Scope(location, error))?;
        }

        Ok(())
    }

    fn evaluate_list_expression(&mut self, list: Vec<Expression>) -> Result<usize, Error> {
        log::trace!("List expression        : {:?}", list);

        let input_length = list.len();
        for expression in list.into_iter() {
            self.evaluate_expression(expression)?;
        }

        Ok(input_length)
    }

    fn evaluate_operand(&mut self, is_for_stack: bool) -> Result<Element, Error> {
        Ok(match self.pop_operand() {
            StackElement::Operand(operand) => {
                match operand {
                    ExpressionOperand::Literal(literal) => self.evaluate_literal(literal)?,
                    ExpressionOperand::Identifier(identifier) => {
                        self.evaluate_identifier(identifier, is_for_stack)?
                    }
                    ExpressionOperand::Type(r#type) => self.evaluate_type(r#type)?,
                    ExpressionOperand::Block(block) => {
                        self.push_scope();
                        self.evaluate_block_expression(block)?;
                        self.scope().borrow_mut().move_assignments();
                        self.pop_scope();
                    }
                    ExpressionOperand::Conditional(conditional) => {
                        self.evaluate_conditional_expression(conditional)?;
                    }
                    ExpressionOperand::List(list) => {
                        self.evaluate_list_expression(list)?;
                    }
                    _ => unimplemented!(),
                }
                match self.pop_operand() {
                    StackElement::Element(element) => element,
                    _ => panic!("Always checked by some branches above"),
                }
            }
            StackElement::Element(element) => element,
        })
    }

    fn get_unary_operand(&mut self, is_for_stack: bool) -> Result<Element, Error> {
        self.evaluate_operand(is_for_stack)
    }

    fn get_binary_operands(
        &mut self,
        is_for_stack_1: bool,
        is_for_stack_2: bool,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand(is_for_stack_2)?;
        let operand_1 = self.evaluate_operand(is_for_stack_1)?;
        Ok((operand_1, operand_2))
    }

    fn push_operand(&mut self, operand: StackElement) {
        log::trace!("!!! {:?}", operand);
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands.pop().expect("Always contains an element")
    }

    fn push_instruction(&mut self, instruction: Box<dyn Instruction>) {
        log::trace!(">>> {:?}", instruction);
        self.instructions.push(instruction);
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scopes
            .last()
            .cloned()
            .expect("Always contains an element")
    }

    fn push_scope(&mut self) {
        self.scopes
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
