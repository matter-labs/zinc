//!
//! The interpreter.
//!

mod element;
mod error;
mod scope;

pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::IntegerError;
pub use self::element::Place;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::scope::Error as ScopeError;
pub use self::scope::Scope;

use std::cell::RefCell;
use std::rc::Rc;

use bellman::ConstraintSystem;
use pairing::bn256::Bn256;
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
    rpn_stack: Vec<Element>,
    loop_stack: Rc<RefCell<Vec<String>>>,
    id_sequence: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new(
            Scope::new(None),
            Rc::new(RefCell::new(Vec::with_capacity(64))),
        )
    }
}

impl Interpreter {
    pub fn new(scope: Scope, loop_stack: Rc<RefCell<Vec<String>>>) -> Self {
        Self {
            system: TestConstraintSystem::new(),
            scope: Rc::new(RefCell::new(scope)),
            rpn_stack: Vec::with_capacity(64),
            loop_stack,
            id_sequence: 0,
        }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for input in program.inputs.into_iter() {
            let location = input.location;
            let namespace = self.system.namespace(|| &input.identifier.name);
            self.scope
                .borrow_mut()
                .declare_variable(Place::new(
                    input.identifier,
                    Value::new_input(input.r#type, namespace)
                        .map_err(|error| Error::Element(location, ElementError::Value(error)))?,
                    false,
                ))
                .map_err(|error| Error::Scope(location, error))?;
        }
        for witness in program.witnesses.into_iter() {
            let location = witness.location;
            let namespace = self.system.namespace(|| &witness.identifier.name);
            self.scope
                .borrow_mut()
                .declare_variable(Place::new(
                    witness.identifier,
                    Value::new_witness(witness.r#type, namespace)
                        .map_err(|error| Error::Element(location, ElementError::Value(error)))?,
                    false,
                ))
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
            Statement::Let(r#let) => {
                let location = r#let.location;
                let value = self.evaluate(r#let.expression)?;
                let value = if let Some(r#type) = r#let.r#type {
                    match (value, r#type.variant) {
                        (value @ Value::Void, TypeVariant::Void) => value,
                        (value @ Value::Boolean(_), TypeVariant::Bool) => value,
                        (Value::Integer(integer), type_variant) => {
                            let namespace = r#let.identifier.name.clone();
                            let namespace = self.system.namespace(|| namespace);
                            let integer =
                                integer.cast(type_variant, namespace).map_err(|error| {
                                    Error::Element(location, ElementError::Integer(error))
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

                let place = Place::new(r#let.identifier, value, r#let.is_mutable);
                self.scope
                    .borrow_mut()
                    .declare_variable(place)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Require(require) => match self.evaluate(require.expression)? {
                Value::Boolean(boolean) => {
                    if boolean.get_value().expect("Always returns a value") {
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

                let namespace = self.next_temp_namespace();
                let namespace = self.system.namespace(|| namespace);
                let range_start = match Value::new_integer(r#loop.range_start, namespace)
                    .map_err(|error| Error::Element(location, ElementError::Value(error)))?
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

                let namespace = self.next_temp_namespace();
                let namespace = self.system.namespace(|| namespace);
                let range_end = match Value::new_integer(r#loop.range_end, namespace)
                    .map_err(|error| Error::Element(location, ElementError::Value(error)))?
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

                let mut index = if range_start.has_the_same_type_as(&range_end) {
                    range_start
                } else {
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    range_start
                        .cast(range_end.type_variant(), namespace)
                        .map_err(|error| Error::Element(location, ElementError::Integer(error)))?
                };

                let namespace = self.next_temp_namespace();
                let namespace = self.system.namespace(|| namespace);
                let is_greater = index
                    .greater(&range_end, namespace)
                    .map_err(|error| Error::Element(location, ElementError::Integer(error)))?
                    .get_value()
                    .expect("Always returns a value");
                if is_greater {
                    return Err(Error::LoopRangeInvalid(location, index, range_end));
                }

                loop {
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    let is_greater_or_equals = index
                        .greater_equals(&range_end, namespace)
                        .map_err(|error| Error::Element(location, ElementError::Integer(error)))?
                        .get_value()
                        .expect("Always returns a value");
                    if is_greater_or_equals {
                        break;
                    }

                    let mut scope = Scope::new(Some(self.scope.clone()));
                    let place = Place::new(
                        r#loop.index_identifier.clone(),
                        Value::Integer(index.clone()),
                        false,
                    );
                    scope
                        .declare_variable(place)
                        .map_err(|error| Error::Scope(location, error))?;

                    let mut executor = Interpreter::new(scope, self.loop_stack.clone());
                    for statement in r#loop.block.statements.iter() {
                        executor.execute(statement.to_owned())?;
                    }
                    if let Some(ref expression) = r#loop.block.expression {
                        executor.evaluate(*expression.to_owned())?;
                    }

                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    index = index
                        .inc(namespace)
                        .map_err(|error| Error::Element(location, ElementError::Integer(error)))?;
                }
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression)?;
                println!("{}", result);
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
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Element::Value(Value::new_boolean(literal, namespace).map_err(
                                    |error| Error::Element(location, ElementError::Value(error)),
                                )?)
                            }
                            lexical::Literal::Integer(literal) => {
                                let location = element.location;
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Element::Value(Value::new_integer(literal, namespace).map_err(
                                    |error| Error::Element(location, ElementError::Value(error)),
                                )?)
                            }
                            literal @ lexical::Literal::String(..) => {
                                return Err(Error::LiteralCannotBeEvaluated(
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
                                .get_variable(&identifier.name)
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
                    self.rpn_stack.push(element);
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    self.scope
                        .borrow_mut()
                        .update_variable(
                            element_1
                                .assign(element_2)
                                .map_err(|error| Error::Element(element.location, error))?,
                        )
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(Element::Value(Value::Void));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .or(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .xor(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .and(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .equals(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .not_equals(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .greater_equals(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .lesser_equals(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .greater(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .lesser(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .add(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .subtract(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .multiply(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .divide(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .modulo(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
                    let (element_2, element_1) = (
                        self.rpn_stack.pop().expect("Always contains an element"),
                        self.rpn_stack.pop().expect("Always contains an element"),
                    );
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .cast(element_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let element_1 = self.rpn_stack.pop().expect("Always contains an element");
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .negate(namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let element_1 = self.rpn_stack.pop().expect("Always contains an element");
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        element_1
                            .not(namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
            }
        }

        match self.rpn_stack.pop() {
            Some(Element::Value(value)) => Ok(value),
            Some(Element::Place(place)) => Ok(place.value),
            _ => panic!("Type expressions cannot be evaluated"),
        }
    }

    fn evaluate_block(&mut self, block: BlockExpression) -> Result<Value, Error> {
        log::trace!("Block expression       : {}", block);

        let mut executor = Interpreter::new(
            Scope::new(Some(self.scope.clone())),
            self.loop_stack.clone(),
        );
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

        if result.get_value().expect("Always returns a value") {
            let mut executor = Interpreter::new(
                Scope::new(Some(self.scope.clone())),
                self.loop_stack.clone(),
            );
            executor.evaluate_block(conditional.main_block)
        } else if let Some(else_if) = conditional.else_if {
            let mut executor = Interpreter::new(
                Scope::new(Some(self.scope.clone())),
                self.loop_stack.clone(),
            );
            executor.evaluate_conditional(*else_if)
        } else if let Some(else_block) = conditional.else_block {
            let mut executor = Interpreter::new(
                Scope::new(Some(self.scope.clone())),
                self.loop_stack.clone(),
            );
            executor.evaluate_block(else_block)
        } else {
            Ok(Value::Void)
        }
    }

    fn next_temp_namespace(&mut self) -> String {
        self.id_sequence += 1;
        format!("temp_{0:06}", self.id_sequence)
    }
}
