//!
//! The interpreter.
//!

mod element;
mod error;
mod scope;

pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Place;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::scope::Error as ScopeError;
pub use self::scope::Scope;

use std::cell::RefCell;
use std::rc::Rc;

use ff::PrimeField;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use sapling_crypto::circuit::test::TestConstraintSystem;

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

pub struct Interpreter {
    system: TestConstraintSystem<Bn256>,
    scope: Rc<RefCell<Scope>>,
    stack: Vec<Element>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new(Scope::new(None))
    }
}

impl Interpreter {
    pub fn new(scope: Scope) -> Self {
        Self {
            system: TestConstraintSystem::new(),
            scope: Rc::new(RefCell::new(scope)),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for input in program.inputs.into_iter() {
            let location = input.location;
            self.scope
                .borrow_mut()
                .declare_input(input.clone(), &mut self.system)
                .map_err(|error| Error::Scope(location, error))?;
            jab::allocate_input(
                &mut self.system,
                || Ok(Fr::from_str("0").expect("Unreachable")),
                input.bitlength(),
            )
            .map_err(|error| Error::Synthesis(location, error.to_string()))?;
        }
        for witness in program.witnesses.into_iter() {
            let location = witness.location;
            self.scope
                .borrow_mut()
                .declare_witness(witness.clone(), &mut self.system)
                .map_err(|error| Error::Scope(location, error))?;
            jab::allocate_witness(
                &mut self.system,
                || Ok(Fr::from_str("0").expect("Unreachable")),
                witness.bitlength(),
            )
            .map_err(|error| Error::Synthesis(location, error.to_string()))?;
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
                let location = r#let.location;
                let value = self.evaluate(r#let.expression)?;
                let value = if let Some(r#type) = r#let.r#type {
                    match (value, r#type.variant) {
                        (value @ Value::Void, TypeVariant::Void) => value,
                        (value @ Value::Boolean(_), TypeVariant::Bool) => value,
                        (Value::Integer(integer), type_variant) => Value::Integer(
                            jab::casting(&mut self.system, &integer, 0)
                                .map_err(|error| Error::Synthesis(location, error.to_string()))?,
                        ),
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

                let place = Place::new(r#let.identifier, value, r#let.is_mutable);
                self.scope
                    .borrow_mut()
                    .declare_variable(place)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Require(require) => match self.evaluate(require.expression)? {
                Value::Boolean(boolean) => {
                    if boolean.get_value().expect("BUG SITE #1") {
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
                let location = r#loop.location;
                let range_start = match Value::new_integer(r#loop.range_start, &mut self.system)
                    .map_err(|error| Error::Synthesis(location, error.to_string()))?
                {
                    Value::Integer(integer) => integer,
                    value => {
                        return Err(Error::Element(
                            location,
                            ElementError::ExpectedIntegerValue(
                                OperatorExpressionOperator::Range,
                                Element::Value(value),
                            ),
                        ))
                    }
                };
                let range_end = match Value::new_integer(r#loop.range_end, &mut self.system)
                    .map_err(|error| Error::Synthesis(location, error.to_string()))?
                {
                    Value::Integer(integer) => integer,
                    value => {
                        return Err(Error::Element(
                            location,
                            ElementError::ExpectedIntegerValue(
                                OperatorExpressionOperator::Range,
                                Element::Value(value),
                            ),
                        ))
                    }
                };

                let mut index = if true
                /*range_start.has_the_same_type_as(&range_end)*/
                {
                    range_start
                } else {
                    jab::casting(&mut self.system, &range_start, 0)
                        .map_err(|error| Error::Synthesis(location, error.to_string()))?
                };

                if jab::greater(&mut self.system, &index, &range_end, 0)
                    .map_err(|error| Error::Synthesis(location, error.to_string()))?
                    .get_value()
                    .expect("BUG SITE #2")
                {
                    return Err(Error::LoopRangeInvalid(
                        location,
                        Value::Integer(index),
                        Value::Integer(range_end),
                    ));
                }

                while jab::lesser(&mut self.system, &index, &range_end, 0)
                    .map_err(|error| Error::Synthesis(location, error.to_string()))?
                    .get_value()
                    .expect("BUG SITE #3")
                {
                    let mut scope = Scope::new(Some(self.scope.clone()));
                    let place = Place::new(
                        r#loop.index_identifier.clone(),
                        Value::Integer(index.clone()),
                        false,
                    );
                    scope
                        .declare_variable(place)
                        .map_err(|error| Error::Scope(location, error))?;

                    let mut executor = Interpreter::new(scope);
                    for statement in r#loop.block.statements.clone().into_iter() {
                        executor.execute(statement)?;
                    }
                    if let Some(expression) = r#loop.block.expression.clone() {
                        executor.evaluate(*expression)?;
                    }
                    let one = jab::allocate_number(&mut self.system, "1")
                        .map_err(|error| Error::Synthesis(location, error.to_string()))?;
                    index = jab::addition(&mut self.system, &index, &one, 0)
                        .map_err(|error| Error::Synthesis(location, error.to_string()))?
                        .0;
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
                                let location = element.location;
                                Element::Value(
                                    Value::new_boolean(literal, &mut self.system).map_err(
                                        |error| {
                                            Error::Element(location, ElementError::Value(error))
                                        },
                                    )?,
                                )
                            }
                            lexical::Literal::Integer(literal) => {
                                let location = element.location;
                                Element::Value(
                                    Value::new_integer(literal, &mut self.system).map_err(
                                        |error| {
                                            Error::Element(location, ElementError::Value(error))
                                        },
                                    )?,
                                )
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
                                .assign(element_2, &mut self.system)
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
                            .or(element_2, &mut self.system)
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
                            .xor(element_2, &mut self.system)
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
                            .and(element_2, &mut self.system)
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
                            .equal(element_2, &mut self.system)
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
                            .not_equal(element_2, &mut self.system)
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
                            .greater_equal(element_2, &mut self.system)
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
                            .lesser_equal(element_2, &mut self.system)
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
                            .greater(element_2, &mut self.system)
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
                            .lesser(element_2, &mut self.system)
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
                            .add(element_2, &mut self.system)
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
                            .subtract(element_2, &mut self.system)
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
                            .multiply(element_2, &mut self.system)
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
                            .divide(element_2, &mut self.system)
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
                            .modulo(element_2, &mut self.system)
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
                            .cast(element_2, &mut self.system)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let element_1 = self.stack.pop().expect("Option state bug");
                    self.stack.push(
                        element_1
                            .negate(&mut self.system)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let element_1 = self.stack.pop().expect("Option state bug");
                    self.stack.push(
                        element_1
                            .not(&mut self.system)
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
        for statement in block.statements.into_iter() {
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

        if result.get_value().expect("BUG SITE #4") {
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
