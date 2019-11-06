//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zrust_bytecode::Instruction;
use zrust_bytecode::Push;

use crate::error::Error as CompilerError;
use crate::lexical::Literal as InnerLiteral;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error;
use crate::semantic::Scope;
use crate::semantic::Value;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Literal;
use crate::syntax::Statement;
use crate::CircuitProgram;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    rpn_stack: Vec<Element>,
    instructions: Vec<Box<dyn Instruction>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_RPN_INITIAL_CAPACITY: usize = 16;
    const INSTRUCTIONS_INITIAL_CAPACITY: usize = 1024;

    pub fn new(scope: Scope) -> Self {
        let mut scope_stack = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
        scope_stack.push(Rc::new(RefCell::new(scope)));

        let rpn_stack = Vec::with_capacity(Self::STACK_RPN_INITIAL_CAPACITY);

        let instructions = Vec::with_capacity(Self::INSTRUCTIONS_INITIAL_CAPACITY);

        Self {
            scope_stack,
            rpn_stack,
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
            Statement::Expression(expression) => self.evaluate_expression(expression)?,
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<(), Error> {
        log::trace!("Operator expression    : {}", expression);

        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => match operand {
                    ExpressionOperand::Literal(literal) => {
                        let element = self.evaluate_literal(literal)?;
                        let push: Push = element.clone().into();
                        self.instructions.push(Box::new(push));
                        self.rpn_stack.push(element);
                    }
                    ExpressionOperand::Type(r#type) => {
                        let element = Element::Type(r#type.variant);
                        self.rpn_stack.push(element);
                    }
                    _ => unimplemented!(),
                },
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .add(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    self.instructions.push(Box::new(zrust_bytecode::Add));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .subtract(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    self.instructions.push(Box::new(zrust_bytecode::Sub));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .multiply(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    self.instructions.push(Box::new(zrust_bytecode::Mul));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .divide(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    self.instructions.push(Box::new(zrust_bytecode::Div));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .modulo(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    self.instructions.push(Box::new(zrust_bytecode::Rem));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let result = operand_1
                        .cast(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand();
                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.rpn_stack.push(result);
                    //                    self.instructions.push(zrust_bytecode::::Negate);
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    unimplemented!();
                }
                _ => unimplemented!(),
            }
        }

        Ok(())
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Result<Element, Error> {
        log::trace!("Literal                : {}", literal);
        let location = literal.location;

        Ok(match literal.data {
            InnerLiteral::Boolean(literal) => {
                Element::Value(Value::new_boolean_from_literal(literal))
            }
            InnerLiteral::Integer(literal) => Value::new_integer_from_literal(literal, None)
                .map(Element::Value)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
            InnerLiteral::String { .. } => panic!("String literals cannot be used in expressions"),
        })
    }

    fn get_operand(&mut self) -> Element {
        self.rpn_stack.pop().expect("Always contains an element")
    }

    fn get_unary_operand(&mut self) -> Element {
        self.get_operand()
    }

    fn get_binary_operands(&mut self) -> (Element, Element) {
        let operand_2 = self.get_operand();
        let operand_1 = self.get_operand();
        (operand_1, operand_2)
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect("Always contains an element")
    }

    fn push_scope(&mut self) {
        self.scope_stack
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }
}
