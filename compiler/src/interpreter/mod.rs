//!
//! The interpreter.
//!

mod element;
mod error;
mod scope;
mod tests;

pub use self::element::Array;
pub use self::element::ArrayError;
pub use self::element::Boolean;
pub use self::element::BooleanError;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::IntegerError;
pub use self::element::Place;
pub use self::element::PlaceError;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::scope::Error as ScopeError;
pub use self::scope::Scope;

use std::cell::RefCell;
use std::rc::Rc;

use r1cs::Bn256;
use r1cs::ConstraintSystem;
use r1cs::TestConstraintSystem;

use crate::lexical;
use crate::syntax::ArrayExpression;
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

pub struct Interpreter {
    system: TestConstraintSystem<Bn256>,
    scope: Rc<RefCell<Scope>>,
    rpn_stack: Vec<Element>,
    id_sequence: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl Interpreter {
    pub fn new(scope: Scope) -> Self {
        Self {
            system: TestConstraintSystem::new(),
            scope: Rc::new(RefCell::new(scope)),
            rpn_stack: Vec::with_capacity(64),
            id_sequence: 0,
        }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for input in program.inputs.into_iter() {
            let location = input.location;
            self.scope
                .borrow_mut()
                .declare_variable(input.identifier.name, Value::Void, false)
                .map_err(|error| Error::Scope(location, error))?;
        }
        for witness in program.witnesses.into_iter() {
            let location = witness.location;
            self.scope
                .borrow_mut()
                .declare_variable(witness.identifier.name, Value::Void, false)
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
                        (Value::Integer(integer), type_variant) => {
                            let namespace = r#let.identifier.name.clone();
                            let namespace = self.system.namespace(|| namespace);
                            integer
                                .cast(namespace, type_variant)
                                .map(Value::Integer)
                                .map_err(|error| Error::LetImplicitCasting(location, error))?
                        }
                        (value, type_variant) => {
                            if value.type_variant() == type_variant {
                                value
                            } else {
                                return Err(Error::LetInvalidType(
                                    r#let.location,
                                    value,
                                    type_variant,
                                ));
                            }
                        }
                    }
                } else {
                    value
                };

                self.scope
                    .borrow_mut()
                    .declare_variable(r#let.identifier.name, value, r#let.is_mutable)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Require(require) => match self.evaluate(require.expression)? {
                Value::Boolean(boolean) => {
                    if boolean.is_true() {
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

                let is_reverse = r#loop.range_end < r#loop.range_start;
                let mut index = r#loop.range_start;

                loop {
                    if match (r#loop.is_range_inclusive, is_reverse) {
                        (true, true) => index < r#loop.range_end,
                        (true, false) => index > r#loop.range_end,
                        (false, true) => index <= r#loop.range_end,
                        (false, false) => index >= r#loop.range_end,
                    } {
                        break;
                    }

                    let mut scope = Scope::new(Some(self.scope.clone()));
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    scope
                        .declare_variable(
                            r#loop.index_identifier.name.clone(),
                            Value::Integer(
                                Integer::new_from_usize(namespace, index)
                                    .map_err(|error| Error::LoopIterator(location, error))?,
                            ),
                            false,
                        )
                        .map_err(|error| Error::Scope(location, error))?;

                    let mut executor = Interpreter::new(scope);
                    if let Some(while_condition) = r#loop.while_condition.clone() {
                        let location = while_condition.location();
                        match executor.evaluate(while_condition)? {
                            Value::Boolean(boolean) => {
                                if boolean.is_false() {
                                    break;
                                }
                            }
                            value => {
                                return Err(Error::LoopWhileExpectedBooleanExpression(
                                    location, value,
                                ))
                            }
                        }
                    }
                    for statement in r#loop.block.statements.iter() {
                        executor.execute(statement.to_owned())?;
                    }
                    if let Some(ref expression) = r#loop.block.expression {
                        executor.evaluate(*expression.to_owned())?;
                    }

                    if is_reverse {
                        index -= 1;
                    } else {
                        index += 1;
                    }
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

        let location = expression.location;
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
                                Element::Value(
                                    Value::new_boolean(namespace, literal)
                                        .map_err(ElementError::Value)
                                        .map_err(|error| Error::Element(location, error))?,
                                )
                            }
                            lexical::Literal::Integer(literal) => {
                                let location = element.location;
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Element::Value(
                                    Value::new_integer(namespace, literal)
                                        .map_err(ElementError::Value)
                                        .map_err(|error| Error::Element(location, error))?,
                                )
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
                            Element::Place(Place::new(identifier.name))
                        }
                        OperatorExpressionOperand::Block(block) => {
                            Element::Value(self.evaluate_block(block)?)
                        }
                        OperatorExpressionOperand::Conditional(conditional) => {
                            Element::Value(self.evaluate_conditional(conditional)?)
                        }
                        OperatorExpressionOperand::Array(array) => {
                            Element::Value(self.evaluate_array(array)?)
                        }
                    };
                    self.rpn_stack.push(element);
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let (place, value) = operand_1
                        .assign(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.scope
                        .borrow_mut()
                        .update_value(&place, value)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(Element::Value(Value::Void));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .or(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .xor(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .and(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .equals(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .not_equals(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .greater_equals(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .lesser_equals(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .greater(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .lesser(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .add(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .subtract(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .multiply(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .divide(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .modulo(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(true, false)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .cast(operand_2, namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let operand_1 = self
                        .get_unary_operand(true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .negate(namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let operand_1 = self
                        .get_unary_operand(true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    self.rpn_stack.push(
                        operand_1
                            .not(namespace)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Indexing) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(
                        operand_1
                            .index(operand_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
            }
        }

        match self.rpn_stack.pop() {
            Some(Element::Value(value)) => Ok(value),
            Some(Element::Place(place)) => self
                .scope
                .borrow()
                .get_value(&place)
                .map_err(|error| Error::Scope(location, error)),
            Some(Element::Type(..)) => panic!("Type expression cannot be the expression result"),
            None => panic!("Always contains an element"),
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

    fn evaluate_array(&mut self, array: ArrayExpression) -> Result<Value, Error> {
        log::trace!("Array expression       : {}", array);

        let location = array.location;

        let mut result = Array::with_capacity(array.elements.len());
        for element in array.elements.into_iter() {
            result
                .push(self.evaluate(element)?)
                .map_err(|error| Error::ArrayLiteral(location, error))?;
        }
        Ok(Value::Array(result))
    }

    fn get_unary_operand(&mut self, resolve: bool) -> Result<Element, ScopeError> {
        self.get_operand(resolve)
    }

    fn get_binary_operands(
        &mut self,
        resolve_1: bool,
        resolve_2: bool,
    ) -> Result<(Element, Element), ScopeError> {
        let operand_2 = self.get_operand(resolve_2)?;
        let operand_1 = self.get_operand(resolve_1)?;
        Ok((operand_1, operand_2))
    }

    fn get_operand(&mut self, resolve: bool) -> Result<Element, ScopeError> {
        let operand = self.rpn_stack.pop().expect("Always contains an element");
        if resolve {
            match operand {
                Element::Place(ref place) => {
                    self.scope.borrow().get_value(place).map(Element::Value)
                }
                Element::Value(value) => Ok(Element::Value(value)),
                Element::Type(..) => panic!("Type expression cannot be resolved"),
            }
        } else {
            Ok(operand)
        }
    }

    fn next_temp_namespace(&mut self) -> String {
        self.id_sequence += 1;
        format!("temp_{0:06}", self.id_sequence)
    }
}
