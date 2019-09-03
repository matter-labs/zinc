//!
//! The interpreter executor.
//!

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::interpreter::Error;
use crate::interpreter::Place;
use crate::interpreter::Scope;
use crate::interpreter::StackElement;
use crate::interpreter::Value;
use crate::interpreter::Warning;
use crate::lexical::Literal;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionObject;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Statement;
use crate::syntax::TypeVariant;

#[derive(Default)]
pub struct Executor {
    stack: Vec<StackElement>,
    scope: Rc<RefCell<Scope>>,
}

impl Executor {
    pub fn new(scope: Scope) -> Self {
        Self {
            stack: Default::default(),
            scope: Rc::new(RefCell::new(scope)),
        }
    }

    pub fn execute(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression)?;
                println!("{}", result);
            }
            Statement::Let(r#let) => {
                if self.scope.borrow().is_variable_declared(&r#let.identifier) {
                    log::warn!(
                        "{}",
                        Warning::RedeclaredVariable(
                            r#let.identifier.location,
                            unsafe { str::from_utf8_unchecked(&r#let.identifier.name) }.to_owned(),
                        )
                    );
                }
                let mut result = self.evaluate(r#let.expression)?;
                if let Some(r#type) = r#let.r#type {
                    result = result
                        .cast(r#type.variant)
                        .map_err(|error| Error::Operator(r#type.location, error))?
                }
                let place = Place::new(r#let.identifier.clone(), result, r#let.is_mutable);
                self.scope.borrow_mut().declare_variable(place);
            }
            Statement::Require(require) => {
                let result = self.evaluate(require.expression)?;
                if result.field.is_zero() {
                    return Err(Error::RequireFailure(require.location, require.id));
                }
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Operator(expression) => self.evaluate_operator(expression),
            Expression::Block(expression) => self.evaluate_block(expression),
        }
    }

    pub fn evaluate_operator(&mut self, expression: OperatorExpression) -> Result<Value, Error> {
        for expression_element in expression.into_iter() {
            match expression_element.object {
                OperatorExpressionObject::Operand(operand) => {
                    let element = match operand {
                        OperatorExpressionOperand::Literal(Literal::Boolean(literal)) => {
                            StackElement::Value(Value::from(literal))
                        }
                        OperatorExpressionOperand::Literal(Literal::Integer(literal)) => {
                            StackElement::Value(Value::from(literal))
                        }
                        OperatorExpressionOperand::Literal(Literal::String(literal)) => {
                            return Err(Error::StringLiteralNotSupported(
                                expression_element.token.location,
                                literal,
                            ));
                        }
                        OperatorExpressionOperand::Type(r#type) => StackElement::Type(r#type),
                        OperatorExpressionOperand::Identifier(identifier) => {
                            let location = expression_element.token.location;
                            self.scope
                                .borrow()
                                .get_variable(&identifier)
                                .map(StackElement::Place)
                                .ok_or_else(|| {
                                    Error::UndeclaredVariable(
                                        location,
                                        unsafe { str::from_utf8_unchecked(&identifier.name) }
                                            .to_owned(),
                                    )
                                })?
                        }
                        OperatorExpressionOperand::Block(block) => {
                            StackElement::Value(self.evaluate_block(block)?)
                        }
                    };
                    self.stack.push(element);
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    if let (Some(element_2), Some(element_1)) = (self.stack.pop(), self.stack.pop())
                    {
                        let place = element_1.assign(element_2).map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;

                        if !self.scope.borrow_mut().update_variable(place) {
                            panic!("Is checked in the operand branch");
                        }

                        self.stack.push(StackElement::Value(Value::new(
                            BigInt::zero(),
                            TypeVariant::Void,
                        )));
                    } else {
                        unreachable!();
                    }
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    if let Some(element) = self.stack.pop() {
                        let result = element.negate().map_err(move |error| {
                            Error::Operator(expression_element.token.location, error)
                        })?;
                        self.stack.push(result);
                    } else {
                        unreachable!();
                    }
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
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

    pub fn evaluate_block(&mut self, block: BlockExpression) -> Result<Value, Error> {
        let mut executor = Executor::new(Scope::new(Some(self.scope.clone())));
        for statement in block.statements {
            executor.execute(statement)?;
        }
        if let Some(expression) = block.expression {
            executor.evaluate(*expression)
        } else {
            Ok(Value::new(BigInt::zero(), TypeVariant::Void))
        }
    }
}
