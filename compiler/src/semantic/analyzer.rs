//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Literal as InnerLiteral;
use crate::semantic::Error;
use crate::semantic::Scope;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Literal;
use crate::syntax::Statement;
use crate::CircuitProgram;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl Analyzer {
    pub fn new(scope: Scope) -> Self {
        let mut scope_stack = Vec::with_capacity(16);
        scope_stack.push(Rc::new(RefCell::new(scope)));
        Self { scope_stack }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for statement in program.statements.into_iter() {
            self.execute_statement(statement)?;
        }

        Ok(())
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
                    ExpressionOperand::Literal(literal) => self.evaluate_literal(literal)?,
                    _ => unimplemented!(),
                },
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    unimplemented!();
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
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    unimplemented!();
                }
                _ => unimplemented!(),
            }
        }

        Ok(())
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Result<(), Error> {
        log::trace!("Literal                : {}", literal);

        match literal.data {
            InnerLiteral::Boolean(literal) => log::info!("push {}", literal),
            InnerLiteral::Integer(literal) => log::info!("push {}", literal),
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        }

        Ok(())
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
