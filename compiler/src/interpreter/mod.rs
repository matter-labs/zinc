//!
//! The interpreter.
//!

mod element;
mod error;
mod scope;
mod warning;

pub use self::element::Boolean;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::Place;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::scope::Error as ScopeError;
pub use self::scope::Scope;
pub use self::scope::Warning as ScopeWarning;
pub use self::warning::Warning;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::lexical;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::Literal;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionObject;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Statement;
use crate::syntax::TypeVariant;

#[derive(Default)]
pub struct Interpreter {
    stack: Vec<Element>,
    scope: Rc<RefCell<Scope>>,
}

impl Interpreter {
    pub fn new(scope: Scope) -> Self {
        Self {
            stack: Default::default(),
            scope: Rc::new(RefCell::new(scope)),
        }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for input in program.inputs.into_iter() {
            let location = input.location;
            self.scope
                .borrow_mut()
                .declare_input(input)
                .map_err(|error| Error::Scope(location, error))?;
        }
        for witness in program.witnesses.into_iter() {
            let location = witness.location;
            self.scope
                .borrow_mut()
                .declare_witness(witness)
                .map_err(|error| Error::Scope(location, error))?;
        }

        for statement in program.statements.into_iter() {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression)?;
                println!("{}", result);
            }
            Statement::Let(r#let) => {
                let value = self.evaluate(r#let.expression)?;
                let value = if let Some(r#type) = r#let.r#type {
                    match (value, r#type.variant) {
                        (value @ Value::Void, TypeVariant::Void) => value,
                        (value @ Value::Boolean(_), TypeVariant::Bool) => value,
                        (Value::Integer(mut integer), type_variant) => {
                            integer = integer.cast(type_variant).map_err(|error| {
                                Error::Element(r#type.location, ElementError::Value(error))
                            })?;
                            Value::Integer(integer)
                        }
                        (value, type_variant) => {
                            return Err(Error::LetDeclarationInvalidType(
                                r#let.location,
                                value,
                                type_variant,
                            ))
                        }
                    }
                } else {
                    value
                };

                let location = r#let.identifier.location;
                let place = Place::new(r#let.identifier, value, r#let.is_mutable);
                if let Err(warning) = self.scope.borrow_mut().declare_variable(place) {
                    log::warn!("{}", Warning::Scope(location, warning));
                }
            }
            Statement::Require(require) => match self.evaluate(require.expression)? {
                Value::Boolean(boolean) => {
                    if boolean.value {
                        log::info!("require {} passed", require.id)
                    } else {
                        return Err(Error::RequireFailed(require.location, require.id));
                    }
                }
                value => {
                    return Err(Error::RequireExpectedBooleanExpression(
                        require.location,
                        require.id,
                        value,
                    ))
                }
            },
            Statement::Loop(r#loop) => {
                log::trace!("Loop statement         : {}", r#loop);

                let location = r#loop.location;
                let range_start = match Value::try_from(r#loop.range_start) {
                    Ok(Value::Integer(integer)) => integer,
                    Ok(value) => {
                        return Err(Error::Element(
                            location,
                            ElementError::ExpectedIntegerValue(
                                OperatorExpressionOperator::Range,
                                Element::Value(value),
                            ),
                        ))
                    }
                    Err(error) => return Err(Error::Element(location, ElementError::Value(error))),
                };
                let range_end = match Value::try_from(r#loop.range_end) {
                    Ok(Value::Integer(integer)) => integer,
                    Ok(value) => {
                        return Err(Error::Element(
                            location,
                            ElementError::ExpectedIntegerValue(
                                OperatorExpressionOperator::Range,
                                Element::Value(value),
                            ),
                        ))
                    }
                    Err(error) => return Err(Error::Element(location, ElementError::Value(error))),
                };

                let mut index = if range_start.has_the_same_type_as(&range_end) {
                    range_start
                } else {
                    range_start
                        .cast(TypeVariant::uint(range_end.bitlength()))
                        .map_err(|error| Error::Element(location, ElementError::Value(error)))?
                };

                if index
                    .greater(&range_end)
                    .map_err(|error| Error::Element(location, ElementError::Value(error)))?
                    .is_true()
                {
                    return Err(Error::LoopRangeInvalid(location, index, range_end));
                }

                let mut warning_logged = false;
                while index
                    .lesser(&range_end)
                    .map_err(|error| Error::Element(location, ElementError::Value(error)))?
                    .is_true()
                {
                    let mut scope = Scope::new(Some(self.scope.clone()));
                    let place = Place::new(
                        r#loop.index_identifier.clone(),
                        Value::Integer(index.clone()),
                        false,
                    );

                    if let Err(warning) = scope.declare_variable(place) {
                        if !warning_logged {
                            log::warn!("{}", Warning::Scope(location, warning));
                            warning_logged = true;
                        }
                    }

                    let mut executor = Interpreter::new(scope);
                    for statement in r#loop.block.statements.clone() {
                        executor.execute(statement)?;
                    }
                    if let Some(expression) = r#loop.block.expression.clone() {
                        executor.evaluate(*expression)?;
                    }
                    index = index
                        .inc()
                        .map_err(|error| Error::Element(location, ElementError::Value(error)))?;
                }
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Operator(expression) => self.evaluate_operator(expression),
            Expression::Block(expression) => self.evaluate_block(expression),
            Expression::Conditional(expression) => self.evaluate_conditional(expression),
        }
    }

    fn evaluate_operator(&mut self, expression: OperatorExpression) -> Result<Value, Error> {
        log::trace!("Operator expression    : {}", expression);

        for element in expression.into_iter() {
            match element.object {
                OperatorExpressionObject::Operand(operand) => {
                    let element = match operand {
                        OperatorExpressionOperand::Literal(literal) => match literal.data {
                            lexical::Literal::Void => Element::Value(Value::Void),
                            lexical::Literal::Boolean(literal) => {
                                Element::Value(Value::from(literal))
                            }
                            lexical::Literal::Integer(literal) => {
                                let location = element.location;
                                Element::Value(Value::try_from(literal).map_err(|error| {
                                    Error::Element(location, ElementError::Value(error))
                                })?)
                            }
                            literal @ lexical::Literal::String(..) => {
                                return Err(Error::LiteralIsNotSupported(
                                    element.location,
                                    Literal::new(element.location, literal),
                                ))
                            }
                        },
                        OperatorExpressionOperand::Type(r#type) => Element::Type(r#type),
                        OperatorExpressionOperand::Identifier(identifier) => {
                            let location = element.location;
                            self.scope
                                .borrow()
                                .get_variable(&identifier)
                                .map(Element::Place)
                                .map_err(|error| Error::Scope(location, error))?
                        }
                        OperatorExpressionOperand::Block(block) => {
                            Element::Value(self.evaluate_block(block)?)
                        }
                        OperatorExpressionOperand::Conditional(conditional) => {
                            Element::Value(self.evaluate_conditional(conditional)?)
                        }
                    };
                    self.stack.push(element);
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.scope
                        .borrow_mut()
                        .update_variable(
                            element_1
                                .assign(element_2)
                                .map_err(|error| Error::Element(element.location, error))?,
                        )
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.stack.push(Element::Value(Value::Void));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .or(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .xor(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .and(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .equal(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .not_equal(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .greater_equal(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .lesser_equal(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .greater(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .lesser(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .add(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .subtract(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .multiply(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .divide(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .modulo(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
                    let (element_2, element_1) = (
                        self.stack.pop().expect("Option state bug"),
                        self.stack.pop().expect("Option state bug"),
                    );
                    self.stack.push(
                        element_1
                            .cast(element_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let element_1 = self.stack.pop().expect("Option state bug");
                    self.stack.push(
                        element_1
                            .negate()
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let element_1 = self.stack.pop().expect("Option state bug");
                    self.stack.push(
                        element_1
                            .not()
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
            }
        }

        match self.stack.pop() {
            Some(Element::Value(value)) => Ok(value),
            Some(Element::Place(place)) => Ok(place.value),
            _ => panic!("Type expressions cannot be evaluated"),
        }
    }

    fn evaluate_block(&mut self, block: BlockExpression) -> Result<Value, Error> {
        log::trace!("Block expression       : {}", block);

        let mut executor = Interpreter::new(Scope::new(Some(self.scope.clone())));
        for statement in block.statements {
            executor.execute(statement)?;
        }
        if let Some(expression) = block.expression {
            executor.evaluate(*expression)
        } else {
            Ok(Value::Void)
        }
    }

    fn evaluate_conditional(&mut self, conditional: ConditionalExpression) -> Result<Value, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let result = match self.evaluate(*conditional.condition)? {
            Value::Boolean(boolean) => boolean,
            value => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    conditional.location,
                    value,
                ))
            }
        };

        if result.is_true() {
            let mut executor = Interpreter::new(Scope::new(Some(self.scope.clone())));
            executor.evaluate_block(conditional.main_block)
        } else if let Some(else_if) = conditional.else_if {
            let mut executor = Interpreter::new(Scope::new(Some(self.scope.clone())));
            executor.evaluate_conditional(*else_if)
        } else if let Some(else_block) = conditional.else_block {
            let mut executor = Interpreter::new(Scope::new(Some(self.scope.clone())));
            executor.evaluate_block(else_block)
        } else {
            Ok(Value::Void)
        }
    }
}
