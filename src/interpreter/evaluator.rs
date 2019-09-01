//!
//! The interpreter evaluator.
//!

use std::collections::HashMap;
use std::str;

use crate::interpreter::Error;
use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::lexical::Literal;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::Type;

pub struct Evaluator {
    stack: Vec<StackElement>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(1024),
        }
    }
}

impl Evaluator {
    pub fn evaluate(
        &mut self,
        expression: Expression,
        variables: &HashMap<Vec<u8>, Value>,
    ) -> Result<Value, Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => self.stack.push(match operand {
                    ExpressionOperand::Literal(Literal::Boolean(literal)) => {
                        StackElement::Value(Value::from(literal))
                    }
                    ExpressionOperand::Literal(Literal::Integer(literal)) => {
                        StackElement::Value(Value::from(literal))
                    }
                    ExpressionOperand::Literal(Literal::String(_literal)) => {
                        panic!("String literals in expressions are not supported!");
                    }
                    ExpressionOperand::Type(r#type) => StackElement::Type(r#type),
                    ExpressionOperand::Identifier(identifier) => {
                        if let Some(value) = variables.get(&identifier.name) {
                            StackElement::Place(Place::new(identifier.clone(), value.clone()))
                        } else {
                            return Err(Error::UndeclaredVariable(
                                element.token.location,
                                unsafe { str::from_utf8_unchecked(&identifier.name) }.to_owned(),
                            ));
                        }
                    }
                }),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    if let (
                        Some(StackElement::Place(mut place)),
                        Some(StackElement::Value(value)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        place
                            .assign(value)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .or(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .xor(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .and(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .equal(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .not_equal(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .greater_equal(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .lesser_equal(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .greater(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .lesser(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .add(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .subtract(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .multiply(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .divide(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    if let (
                        Some(StackElement::Value(value_2)),
                        Some(StackElement::Value(value_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = value_1
                            .modulo(value_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    if let (Some(StackElement::Type(r#type)), Some(StackElement::Value(value))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        let result = value
                            .cast(r#type.variant)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    if let Some(StackElement::Value(value)) = self.stack.pop() {
                        let result = value
                            .negate()
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    if let Some(StackElement::Value(value)) = self.stack.pop() {
                        let result = value
                            .not()
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Value(result));
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        if let Some(StackElement::Value(value)) = self.stack.pop() {
            Ok(value)
        } else {
            unreachable!();
        }
    }
}

enum StackElement {
    Place(Place),
    Value(Value),
    Type(Type),
}
