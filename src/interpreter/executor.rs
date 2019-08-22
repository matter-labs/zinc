//!
//! The interpreter executor.
//!

use crate::interpreter::Error;
use crate::interpreter::Field;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;

pub struct Executor {
    stack: Vec<Field>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(1024),
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, expression: Expression) -> Result<Field, Error> {
        for element in expression.elements.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => self.stack.push(match operand {
                    ExpressionOperand::Literal(literal) => Field::from(literal),
                    ExpressionOperand::Identifier(_identifier) => unimplemented!(),
                    ExpressionOperand::Type(r#_type) => unimplemented!(),
                }),
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .addition(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .subtraction(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .multiplication(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .division(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .remainder(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand = self.stack.pop().expect("Stack bug");
                    let result = operand
                        .negation()
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .equal(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .not_equal(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .greater_equal(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .lesser_equal(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .greater(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .lesser(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .or(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .xor(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");
                    let result = operand_1
                        .and(operand_2)
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand = self.stack.pop().expect("Stack bug");
                    let result = operand
                        .not()
                        .map_err(move |error| Error::Field(element.token.location, error))?;
                    self.stack.push(result);
                }
                //                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                //                    let operand = self.stack.pop().expect("Stack bug");
                //                    let operand = self.stack.pop().expect("Stack bug");
                //                    let result = operand
                //                        .casting()
                //                        .map_err(move |error| Error::Field(element.token.location, error))?;
                //                    self.stack.push(result);
                //                }
                _ => unimplemented!(),
            }
        }

        Ok(self.stack.pop().expect("Stack bug"))
    }
}
