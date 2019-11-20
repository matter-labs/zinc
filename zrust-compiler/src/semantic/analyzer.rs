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
use crate::semantic::inference;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error;
use crate::semantic::Integer;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::Value;
use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::InnerStatement;
use crate::syntax::Literal;
use crate::syntax::OuterStatement;
use crate::syntax::Type;
use crate::syntax::TypeVariant;
use crate::CircuitProgram;

pub struct Analyzer {
    scopes: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    instructions: Vec<Box<dyn Instruction>>,
    stack_height: usize,
    function_addresses: HashMap<String, usize>,
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
            function_addresses: HashMap::new(),
        }
    }

    pub fn compile(
        mut self,
        program: CircuitProgram,
    ) -> Result<Vec<Box<dyn Instruction>>, CompilerError> {
        self.push_instruction(Box::new(zrust_bytecode::NoOperation));
        self.push_instruction(Box::new(zrust_bytecode::NoOperation));

        for statement in program.statements.into_iter() {
            self.outer_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        let main_function_address = self
            .function_addresses
            .get("main")
            .copied()
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;
        self.instructions[0] = Box::new(zrust_bytecode::Call::new(main_function_address, 0));
        self.instructions[1] = Box::new(zrust_bytecode::Exit);

        Ok(self.instructions)
    }

    fn outer_statement(&mut self, statement: OuterStatement) -> Result<(), Error> {
        match statement {
            OuterStatement::Type(r#type) => {
                let location = r#type.location;
                let type_variant = TypeVariant::new_alias(r#type.identifier.name.clone());
                self.scope()
                    .borrow_mut()
                    .declare_type(r#type.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Struct(r#struct) => {
                let location = r#struct.location;
                let type_variant = TypeVariant::new_structure(
                    r#struct.identifier.clone(),
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
            OuterStatement::Enum(r#enum) => {
                let location = r#enum.location;
                let type_variant = TypeVariant::new_enumeration(
                    r#enum.identifier.clone(),
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
            OuterStatement::Fn(r#fn) => {
                let location = r#fn.location;
                let type_variant = TypeVariant::new_function(
                    r#fn.identifier.clone(),
                    r#fn.arguments
                        .into_iter()
                        .map(|field| (field.identifier.name, field.r#type.variant))
                        .collect(),
                    r#fn.return_type.variant.clone(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#fn.identifier.name.clone(), type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
                self.function_addresses
                    .insert(r#fn.identifier.name.clone(), self.instructions.len());
                self.stack_height = 0;
                self.push_scope();
                self.evaluate_block_expression(r#fn.body)?;
                self.pop_scope();

                let result = self.get_unary_operand(true)?;
                let return_type = result
                    .type_variant(self.scope().borrow().deref())
                    .map_err(|error| Error::Element(location, error))?;
                self.push_operand(StackElement::Element(result));

                if r#fn.return_type.variant != return_type {
                    return Err(Error::FunctionReturnTypeMismatch(
                        r#fn.return_type.location,
                        r#fn.identifier.name,
                        r#fn.return_type.variant,
                        return_type,
                    ));
                }

                match r#fn.return_type.variant {
                    TypeVariant::Unit => {}
                    _ => self.push_instruction(Box::new(zrust_bytecode::Return::new(1))),
                }
            }
            OuterStatement::Mod(_mod) => unimplemented!(),
            OuterStatement::Use(_use) => unimplemented!(),
        }

        Ok(())
    }

    fn inner_statement(&mut self, statement: InnerStatement) -> Result<(), Error> {
        match statement {
            InnerStatement::Empty => {}
            InnerStatement::Let(r#let) => {
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
                    operand_1
                        .type_variant(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(location, error))?
                };

                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        Place::new(r#let.identifier.name),
                        Value::new(type_variant),
                        r#let.is_mutable,
                        self.stack_last_index(),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
            InnerStatement::Loop(r#loop) => {
                let location = r#loop.location;
                let range_bitlength =
                    inference::enough_bitlength(&[&r#loop.range_start, &r#loop.range_end])
                        .map_err(|error| Error::LoopBoundsTypeInference(location, error))?;

                let range_start: usize = r#loop.range_start.into();
                let range_end: usize = r#loop.range_end.into();
                let iterations_number = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if r#loop.is_range_inclusive { 1 } else { 0 };
                let is_reverse = range_start > range_end;
                let index = Integer::new_range_bound(range_start, range_bitlength);

                self.push_instruction(Box::new(index.to_push()));
                self.push_instruction(Box::new(zrust_bytecode::LoopBegin::new(
                    iterations_number,
                    1,
                )));
                self.push_scope();
                let index_address = self.stack_last_index();
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        Place::new(r#loop.index_identifier.name),
                        Value::Integer(index),
                        false,
                        index_address,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                self.evaluate_block_expression(r#loop.block)?;
                self.pop_scope();
                self.push_instruction(Box::new(Integer::new_one(range_bitlength).to_push()));
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(index_address)));
                self.push_instruction(if is_reverse {
                    Box::new(zrust_bytecode::Sub)
                } else {
                    Box::new(zrust_bytecode::Add)
                });
                self.push_instruction(Box::new(zrust_bytecode::LoopEnd));
            }
            InnerStatement::Expression(expression) => {
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
                        .update_variable(place, self.stack_last_index())
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.push_operand(StackElement::Element(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Or));

                    let result = operand_1
                        .or(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Xor));

                    let result = operand_1
                        .xor(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::And));

                    let result = operand_1
                        .and(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Eq));

                    let result = operand_1
                        .equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Ne));

                    let result = operand_1
                        .not_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Gt));

                    let result = operand_1
                        .greater(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Lt));

                    let result = operand_1
                        .lesser(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Add));

                    let result = operand_1
                        .add(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Sub));

                    let result = operand_1
                        .subtract(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Mul));

                    let result = operand_1
                        .multiply(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Div));

                    let result = operand_1
                        .divide(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.get_binary_operands(true, true)?;
                    self.push_instruction(Box::new(zrust_bytecode::Rem));

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
                ExpressionObject::Operator(ExpressionOperator::Indexing) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Field) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, false)?;

                    let (identifier, argument_types, return_type) = match operand_1 {
                        Element::Type(TypeVariant::Function {
                            identifier,
                            arguments,
                            return_type,
                        }) => (identifier, arguments, return_type),
                        operand => {
                            return Err(Error::FunctionCallOnNotCallable(element.location, operand))
                        }
                    };

                    let argument_values = match operand_2 {
                        Element::ValueList(values) => values,
                        _ => panic!("Ensured to be an expression list during the syntax analysis"),
                    };
                    let arguments_types_number = argument_types.len();
                    let arguments_values_number = argument_values.len();
                    if argument_values.len() != argument_types.len() {
                        return Err(Error::FunctionArgumentCountMismatch(
                            element.location,
                            identifier.name,
                            arguments_types_number,
                            arguments_values_number,
                        ));
                    }

                    for (argument_index, (argument_name, expected_type)) in
                        argument_types.into_iter().enumerate()
                    {
                        let actual_type = argument_values[argument_index].type_variant();
                        if expected_type != actual_type {
                            return Err(Error::FunctionArgumentTypeMismatch(
                                element.location,
                                identifier.name,
                                argument_name,
                                expected_type,
                                actual_type,
                            ));
                        }
                    }

                    self.push_instruction(Box::new(zrust_bytecode::Call::new(
                        self.function_addresses
                            .get(&identifier.name)
                            .copied()
                            .expect("Presence checked during the function type resolution"),
                        arguments_values_number,
                    )));

                    self.push_operand(StackElement::Element(Element::Value(Value::new(
                        *return_type,
                    ))))
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => unimplemented!(),
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
        self.push_operand(StackElement::Element(Element::Value(value)));

        Ok(())
    }

    fn evaluate_identifier(
        &mut self,
        identifier: Identifier,
        is_for_stack: bool,
    ) -> Result<(), Error> {
        let location = identifier.location;

        if is_for_stack {
            let place = Place::new(identifier.name.clone());
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
            self.push_instruction(Box::new(zrust_bytecode::Copy::new(address)));
            self.push_operand(StackElement::Element(Element::Value(value)));
        } else {
            let item_type = self
                .scope()
                .borrow()
                .get_item_type(&identifier.name)
                .map_err(|error| Error::Scope(location, error))?;
            match item_type {
                ScopeItem::Variable => {
                    let place = Place::new(identifier.name.clone());
                    self.push_operand(StackElement::Element(Element::Place(place)))
                }
                ScopeItem::Type => {
                    let type_variant = self
                        .scope()
                        .borrow()
                        .resolve_type(&identifier.name)
                        .map_err(|error| Error::Scope(location, error))?;
                    self.push_operand(StackElement::Element(Element::Type(type_variant)));
                }
                ScopeItem::Variant => {}
            }
        }

        Ok(())
    }

    fn evaluate_type(&mut self, r#type: Type) -> Result<(), Error> {
        let location = r#type.location;
        let resolved_type_variant = match r#type.variant {
            TypeVariant::Alias { identifier } => self
                .scope()
                .borrow()
                .resolve_type(&identifier)
                .map_err(|error| Error::Scope(location, error))?,
            type_variant => type_variant,
        };
        self.push_operand(StackElement::Element(Element::Type(resolved_type_variant)));

        Ok(())
    }

    fn evaluate_block_expression(&mut self, block: BlockExpression) -> Result<(), Error> {
        for statement in block.statements.into_iter() {
            self.inner_statement(statement)?;
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
        self.push_instruction(Box::new(zrust_bytecode::PushCondition));
        let condition_address = self.stack_last_index();
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
            (self.get_unary_operand(true)?, self.stack_last_index());
        let main_assignments = self.scope().borrow().get_assignments();
        self.pop_scope();

        let mut else_assignments = HashMap::new();
        let (else_result, else_result_address) = if let Some(else_if) = conditional.else_if {
            self.evaluate_conditional_expression(*else_if)?;
            (self.get_unary_operand(true)?, self.stack_last_index())
        } else if let Some(else_block) = conditional.else_block {
            self.push_scope();
            self.evaluate_block_expression(else_block)?;
            else_assignments = self.scope().borrow().get_assignments();
            self.pop_scope();
            (self.get_unary_operand(true)?, self.stack_last_index())
        } else {
            self.push_operand(StackElement::Element(Element::Value(Value::Unit)));
            (Element::Value(Value::Unit), self.stack_last_index())
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
                .update_variable(place, self.stack_last_index())
                .map_err(|error| Error::Scope(location, error))?;
        }

        match main_type {
            TypeVariant::Unit => {}
            _ => {
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(else_result_address)));
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(main_result_address)));
                self.push_instruction(Box::new(zrust_bytecode::Copy::new(condition_address)));
                self.push_instruction(Box::new(zrust_bytecode::ConditionalSelect));
                self.push_operand(StackElement::Element(main_result));
            }
        }

        self.push_instruction(Box::new(zrust_bytecode::PopCondition));

        Ok(())
    }

    fn evaluate_list_expression(&mut self, list: Vec<Expression>) -> Result<usize, Error> {
        let input_length = list.len();
        let mut values = Vec::with_capacity(input_length);
        for expression in list.into_iter() {
            self.evaluate_expression(expression)?;
            match self.get_unary_operand(true)? {
                Element::Value(value) => values.push(value),
                _ => panic!("Is always put as a value by the evaluator above"),
            }
        }
        self.push_operand(StackElement::Element(Element::ValueList(values)));

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
                    StackElement::Operand { .. } => {
                        panic!("Is always put as an element by the evaluators above")
                    }
                }
            }
            StackElement::Element(element) => element,
        })
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scopes
            .last()
            .cloned()
            .expect("Cannot fail because the global scope has been pushed during initialization")
    }

    fn push_scope(&mut self) {
        self.scopes
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
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
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands
            .pop()
            .expect("Should be called if there are no operands to pop")
    }

    fn push_instruction(&mut self, instruction: Box<dyn Instruction>) {
        //        log::trace!(">>> {:?}", instruction);
        let stack_difference =
            (instruction.outputs_count() as isize) - (instruction.inputs_count() as isize);
        self.stack_resize(stack_difference);
        self.instructions.push(instruction);
    }

    fn stack_resize(&mut self, difference: isize) {
        if difference > 0 {
            self.stack_height += difference as usize;
        } else {
            self.stack_height -= (-difference) as usize;
        }
    }

    fn stack_last_index(&self) -> usize {
        (self.stack_height - 1) as usize
    }
}
