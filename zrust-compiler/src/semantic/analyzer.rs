//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
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
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::Value;
use crate::syntax::BlockExpression;
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
}

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
        log::trace!("Statement              : {}", statement);

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

                    let operand_1 = self.get_unary_operand()?;
                    let (result, bitlength) = operand_1
                        .cast(
                            Element::Type(let_type_variant.clone()),
                            self.scope().borrow().deref(),
                        )
                        .map_err(|error| Error::Element(location, error))?;
                    self.instructions
                        .push(Box::new(zrust_bytecode::Cast::new(bitlength as u8)));
                    self.operands.push(StackElement::Element(result.clone()));
                    let_type_variant
                } else {
                    let operand_1 = self.get_unary_operand()?;
                    operand_1
                        .type_variant(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(location, error))?
                };

                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        r#let.identifier.name,
                        Value::new(type_variant),
                        r#let.is_mutable,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Loop(r#loop) => {
                let location = r#loop.location;
                let enough_bitlength =
                    inference::enough_bitlength(&[&r#loop.range_start, &r#loop.range_end])
                        .map_err(|error| Error::Inference(location, error))?;
                let value = Value::new(TypeVariant::new_integer_unsigned(enough_bitlength));

                let range_start: usize = r#loop.range_start.into();
                let range_end: usize = r#loop.range_end.into();
                let iterations_number = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if r#loop.is_range_inclusive { 1 } else { 0 };

                self.instructions
                    .push(Box::new(zrust_bytecode::LoopBegin::new(iterations_number)));
                self.push_scope();
                //                self.scope()
                //                    .borrow_mut()
                //                    .declare_variable(r#loop.index_identifier.name, value, false)
                //                    .map_err(|error| Error::Scope(location, error))?;
                self.evaluate_block_expression(r#loop.block)?;
                self.pop_scope();
                self.instructions.push(Box::new(zrust_bytecode::LoopEnd));
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
        log::trace!("Operator expression    : {}", expression);

        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.operands.push(StackElement::Operand(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let (place, value) = operand_1
                        .assign(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.instructions.push(Box::new(value.to_push()));
                    self.scope()
                        .borrow_mut()
                        .update_variable(&place, value)
                        .map_err(|error| Error::Scope(element.location, error))?;
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {}
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {}
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Or));

                    let result = operand_1
                        .or(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Xor));

                    let result = operand_1
                        .xor(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::And));

                    let result = operand_1
                        .and(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Eq));

                    let result = operand_1
                        .equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Ne));

                    let result = operand_1
                        .not_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Gt));

                    let result = operand_1
                        .greater(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Lt));

                    let result = operand_1
                        .lesser(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Add));

                    let result = operand_1
                        .add(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Sub));

                    let result = operand_1
                        .subtract(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Mul));

                    let result = operand_1
                        .multiply(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Div));

                    let result = operand_1
                        .divide(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.instructions.push(Box::new(zrust_bytecode::Rem));

                    let result = operand_1
                        .modulo(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let (result, bitlength) = operand_1
                        .cast(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.instructions
                        .push(Box::new(zrust_bytecode::Cast::new(bitlength as u8)));
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand()?;
                    self.instructions.push(Box::new(zrust_bytecode::Neg));

                    let result = operand_1
                        .negate(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand()?;
                    self.instructions.push(Box::new(zrust_bytecode::Not));

                    let result = operand_1
                        .not(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.operands.push(StackElement::Element(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {}
                ExpressionObject::Operator(ExpressionOperator::Field) => {}
                ExpressionObject::Operator(ExpressionOperator::Call) => {}
                ExpressionObject::Operator(ExpressionOperator::Path) => {}
            }
        }

        let element = self.evaluate_operand()?;
        self.operands.push(StackElement::Element(element));
        Ok(())
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Result<(), Error> {
        log::trace!("Literal                : {}", literal);
        let location = literal.location;

        let value = match literal.data {
            InnerLiteral::Boolean(literal) => Value::from(literal),
            InnerLiteral::Integer(literal) => Value::try_from(literal)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
            InnerLiteral::String { .. } => panic!("String literals cannot be used in expressions"),
        };
        self.instructions.push(Box::new(value.to_push()));
        self.operands
            .push(StackElement::Element(Element::Value(value)));

        Ok(())
    }

    fn evaluate_identifier(&mut self, identifier: Identifier) -> Result<(), Error> {
        log::trace!("Identifier             : {}", identifier);

        let place = Place::new(identifier);
        let variable = self
            .scope()
            .borrow()
            .get_variable(&place)
            .map_err(|error| Error::Scope(Location::new(0, 0), error))?;
        self.instructions
            .push(Box::new(zrust_bytecode::Copy::new(variable.address)));
        self.operands
            .push(StackElement::Element(Element::Place(place)));

        Ok(())
    }

    fn evaluate_type(&mut self, r#type: Type) -> Result<(), Error> {
        log::trace!("Type                   : {}", r#type);

        self.operands
            .push(StackElement::Element(Element::Type(r#type.variant)));

        Ok(())
    }

    fn evaluate_block_expression(&mut self, block: BlockExpression) -> Result<(), Error> {
        log::trace!("Block expression       : {}", block);

        for statement in block.statements.into_iter() {
            self.execute_statement(statement)?;
        }
        let local_variable_number = self.scope().borrow().stack_size();
        let pop_position = self.instructions.len();
        if let Some(expression) = block.expression {
            self.evaluate_expression(*expression)?;
        }
        if local_variable_number > 0 {
            self.instructions.insert(
                pop_position,
                Box::new(zrust_bytecode::Pop::new(local_variable_number)),
            );
        }

        Ok(())
    }

    fn evaluate_operand(&mut self) -> Result<Element, Error> {
        Ok(
            match self.operands.pop().expect("Always contains an element") {
                StackElement::Operand(operand) => {
                    match operand {
                        ExpressionOperand::Literal(literal) => self.evaluate_literal(literal)?,
                        ExpressionOperand::Identifier(identifier) => {
                            self.evaluate_identifier(identifier)?
                        }
                        ExpressionOperand::Type(r#type) => self.evaluate_type(r#type)?,
                        ExpressionOperand::Block(block) => {
                            self.push_scope();
                            self.evaluate_block_expression(block)?;
                            self.pop_scope();
                        }
                        _ => unimplemented!(),
                    }
                    match self.operands.pop().expect("Always contains an element") {
                        StackElement::Element(element) => element,
                        _ => panic!("Always checked by some branches above"),
                    }
                }
                StackElement::Element(element) => element,
            },
        )
    }

    fn get_unary_operand(&mut self) -> Result<Element, Error> {
        self.evaluate_operand()
    }

    fn get_binary_operands(&mut self) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand()?;
        let operand_1 = self.evaluate_operand()?;
        Ok((operand_1, operand_2))
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
