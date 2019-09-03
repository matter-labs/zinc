//!
//! The interpreter evaluator.
//!

use std::collections::HashMap;
use std::str;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::interpreter::Error;
use crate::interpreter::Place;
use crate::interpreter::StackElement;
use crate::interpreter::Value;
use crate::lexical::Literal;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeVariant;

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
        variables: &mut HashMap<Vec<u8>, Place>,
    ) -> Result<Value, Error> {
        for expression_element in expression.into_iter() {
            match expression_element.object {
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
                        if let Some(place) = variables.get(&identifier.name).cloned() {
                            StackElement::Place(place)
                        } else {
                            return Err(Error::UndeclaredVariable(
                                expression_element.token.location,
                                unsafe { str::from_utf8_unchecked(&identifier.name) }.to_owned(),
                            ));
                        }
                    }
                }),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let place = element_1.assign(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        let entry = variables
                            .get_mut(&place.identifier.name)
                            .expect("Option state bug");
                        *entry = place;
                        self.stack.push(StackElement::Value(Value::new(
                            BigInt::zero(),
                            TypeVariant::Void,
                        )));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.or(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.xor(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.and(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.equal(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.not_equal(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.greater_equal(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.lesser_equal(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.greater(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.lesser(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.add(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.subtract(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.multiply(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.divide(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.modulo(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = element_1.cast(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    if let Some(element) = self.stack.pop() {
                        let result = element.negate().map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    if let Some(element) = self.stack.pop() {
                        let result = element.not().map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        match self.stack.pop() {
            Some(StackElement::Value(value)) => Ok(value),
            Some(StackElement::Place(place)) => Ok(place.value),
            _ => unreachable!(),
        }
    }
}
