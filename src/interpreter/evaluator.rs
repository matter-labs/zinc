//!
//! The interpreter evaluator.
//!

use std::collections::HashMap;
use std::str;

use crate::interpreter::Error;
use crate::interpreter::Field;
use crate::lexical::Literal;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
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
        variables: &HashMap<Vec<u8>, Field>,
    ) -> Result<Field, Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => self.stack.push(match operand {
                    ExpressionOperand::Literal(Literal::Boolean(literal)) => {
                        StackElement::Field(Field::from(literal))
                    }
                    ExpressionOperand::Literal(Literal::Integer(literal)) => {
                        StackElement::Field(Field::from(literal))
                    }
                    ExpressionOperand::Literal(Literal::String(_literal)) => {
                        panic!("String literals in expressions are not supported!");
                    }
                    ExpressionOperand::Type(r#type) => StackElement::Type(r#type),
                    ExpressionOperand::Identifier(identifier) => {
                        match variables.get(&identifier.name) {
                            Some(value) => StackElement::Field(value.to_owned()),
                            None => {
                                return Err(Error::UndeclaredVariable(
                                    element.token.location,
                                    unsafe { str::from_utf8_unchecked(&identifier.name) }
                                        .to_owned(),
                                ))
                            }
                        }
                    }
                }),
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .add(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .subtract(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .multiply(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .divide(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .modulo(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    if let Some(StackElement::Field(field)) = self.stack.pop() {
                        let result = field
                            .negate()
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .equal(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .not_equal(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .greater_equal(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .lesser_equal(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .greater(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .lesser(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .or(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .xor(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    if let (
                        Some(StackElement::Field(field_2)),
                        Some(StackElement::Field(field_1)),
                    ) = (self.stack.pop(), self.stack.pop())
                    {
                        let result = field_1
                            .and(field_2)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    if let Some(StackElement::Field(field)) = self.stack.pop() {
                        let result = field
                            .not()
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    if let (Some(StackElement::Type(r#type)), Some(StackElement::Field(field))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        let result = field
                            .cast(r#type.variant)
                            .map_err(move |error| Error::Operator(element.token.location, error))?;
                        self.stack.push(StackElement::Field(result));
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        if let Some(StackElement::Field(field)) = self.stack.pop() {
            Ok(field)
        } else {
            unreachable!();
        }
    }
}

enum StackElement {
    Field(Field),
    Type(Type),
}
