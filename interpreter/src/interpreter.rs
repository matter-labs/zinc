//!
//! The interpreter.
//!

use std::cell::RefCell;
use std::rc::Rc;

use parser::ArrayExpression;
use parser::BlockExpression;
use parser::CircuitProgram;
use parser::ConditionalExpression;
use parser::Expression;
use parser::ExpressionObject;
use parser::ExpressionOperand;
use parser::ExpressionOperator;
use parser::InnerLiteral;
use parser::IntegerLiteral;
use parser::Literal;
use parser::MatchExpression;
use parser::Statement;
use parser::StructureExpression;
use parser::TupleExpression;
use parser::TypeVariant;
use r1cs::Bn256;
use r1cs::ConstraintSystem;
use r1cs::TestConstraintSystem;

use crate::element::Element;
use crate::element::Error as ElementError;
use crate::element::Integer;
use crate::element::Place;
use crate::element::Value;
use crate::scope::Error as ScopeError;
use crate::scope::Item as ScopeItem;
use crate::scope::Scope;
use crate::Error;

pub struct Interpreter {
    system: TestConstraintSystem<Bn256>,
    scope_stack: Vec<Rc<RefCell<Scope>>>,
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
        let mut scope_stack = Vec::with_capacity(16);
        scope_stack.push(Rc::new(RefCell::new(scope)));

        Self {
            system: TestConstraintSystem::new(),
            scope_stack,
            rpn_stack: Vec::with_capacity(64),
            id_sequence: 0,
        }
    }

    pub fn interpret(&mut self, program: CircuitProgram) -> Result<(), Error> {
        for input in program.inputs.into_iter() {
            let location = input.location;
            self.scope()
                .borrow_mut()
                .declare_variable(input.identifier.name, Value::Unit, false)
                .map_err(|error| Error::Scope(location, error))?; // TODO
        }
        for witness in program.witnesses.into_iter() {
            let location = witness.location;
            self.scope()
                .borrow_mut()
                .declare_variable(witness.identifier.name, Value::Unit, false)
                .map_err(|error| Error::Scope(location, error))?; // TODO
        }

        for statement in program.statements.into_iter() {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Empty => {}
            Statement::Require(require) => match self.evaluate_expression(require.expression)? {
                Value::Boolean(boolean) => {
                    if boolean.is_true() {
                        log::info!("require '{}' passed", require.annotation);
                    } else {
                        return Err(Error::RequireFailed(require.location, require.annotation));
                    }
                }
                value => {
                    return Err(Error::RequireExpectedBooleanExpression(
                        require.location,
                        require.annotation,
                        value,
                    ))
                }
            },
            Statement::Let(r#let) => {
                let location = r#let.location;
                let value = self.evaluate_expression(r#let.expression)?;
                let value = if let Some(r#type) = r#let.r#type {
                    let let_type_variant = match r#type.variant {
                        TypeVariant::Alias { identifier } => {
                            let location = r#type.location;
                            self.scope()
                                .borrow()
                                .resolve_type(&identifier)
                                .map_err(|error| Error::Scope(location, error))?
                        }
                        type_variant => type_variant,
                    };
                    let value_type_variant = value.type_variant();
                    if let_type_variant == value_type_variant {
                        value
                    } else {
                        match (value, let_type_variant) {
                            (Value::Integer(integer), type_variant) => {
                                let namespace = r#let.identifier.name.clone();
                                let namespace = self.system.namespace(|| namespace);
                                integer
                                    .cast(namespace, type_variant)
                                    .map(Value::Integer)
                                    .map_err(|error| Error::LetImplicitCasting(location, error))?
                            }
                            (value, let_type_variant) => {
                                let value_type_variant = value.type_variant();
                                if value_type_variant == let_type_variant {
                                    value
                                } else {
                                    return Err(Error::LetInvalidType(
                                        r#let.location,
                                        value_type_variant,
                                        let_type_variant,
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    value
                };

                self.scope()
                    .borrow_mut()
                    .declare_variable(r#let.identifier.name, value, r#let.is_mutable)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Loop(r#loop) => {
                let location = r#loop.location;

                let bitlength =
                    semantic::infer_enough_bitlength(&[&r#loop.range_start, &r#loop.range_end])
                        .map_err(|error| Error::BitlengthInference(location, error))?;

                let range_start = r#loop.range_start.into();
                let range_end = r#loop.range_end.into();

                let is_reverse = range_end < range_start;
                let mut index = range_start;

                loop {
                    if match (r#loop.is_range_inclusive, is_reverse) {
                        (true, true) => index < range_end,
                        (true, false) => index > range_end,
                        (false, true) => index <= range_end,
                        (false, false) => index >= range_end,
                    } {
                        break;
                    }

                    self.push_scope();
                    let scope = self.scope();
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    scope
                        .borrow_mut()
                        .declare_variable(
                            r#loop.index_identifier.name.clone(),
                            Value::Integer(
                                Integer::new_from_usize(namespace, index, bitlength)
                                    .map_err(|error| Error::LoopIterator(location, error))?,
                            ),
                            false,
                        )
                        .map_err(|error| Error::Scope(location, error))?;

                    if let Some(while_condition) = r#loop.while_condition.clone() {
                        let location = while_condition.location;
                        match self.evaluate_expression(while_condition)? {
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
                        self.execute_statement(statement.to_owned())?;
                    }
                    if let Some(ref expression) = r#loop.block.expression {
                        self.evaluate_expression(*expression.to_owned())?;
                    }
                    self.pop_scope();

                    if is_reverse {
                        if index > 0 {
                            index -= 1;
                        } else {
                            break;
                        }
                    } else {
                        index += 1;
                    }
                }
            }
            Statement::Type(r#type) => {
                let location = r#type.location;
                self.scope()
                    .borrow_mut()
                    .declare_type(r#type.identifier.name, r#type.r#type.variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Struct(r#struct) => {
                let location = r#struct.location;
                let type_variant = TypeVariant::new_structure(
                    r#struct
                        .fields
                        .into_iter()
                        .map(|(identifier, r#type)| (identifier.name, r#type.variant))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#struct.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Enum(r#enum) => {
                let location = r#enum.location;
                let type_variant = TypeVariant::new_enumeration(
                    r#enum
                        .variants
                        .into_iter()
                        .map(|(identifier, value)| (identifier.name, value))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#enum.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Debug(debug) => {
                let result = self.evaluate_expression(debug.expression)?;
                log::info!("{}", result);
            }
            Statement::Expression(expression) => {
                self.evaluate_expression(expression)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, Error> {
        log::trace!("Operator expression    : {}", expression);

        let location = expression.location;
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    let element = match operand {
                        ExpressionOperand::Unit => Element::Value(Value::Unit),
                        ExpressionOperand::Literal(literal) => match literal.data {
                            InnerLiteral::Boolean(literal) => {
                                let location = element.location;
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Element::Value(
                                    Value::new_boolean_from_literal(namespace, literal)
                                        .map_err(ElementError::Value)
                                        .map_err(|error| Error::Element(location, error))?,
                                )
                            }
                            InnerLiteral::Integer(literal) => {
                                let location = element.location;
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Element::Value(
                                    Value::new_integer_from_literal(namespace, literal, None)
                                        .map_err(ElementError::Value)
                                        .map_err(|error| Error::Element(location, error))?,
                                )
                            }
                            literal @ InnerLiteral::String(..) => {
                                return Err(Error::LiteralCannotBeEvaluated(
                                    element.location,
                                    Literal::new(element.location, literal),
                                ))
                            }
                        },
                        ExpressionOperand::Type(r#type) => Element::Type(r#type),
                        ExpressionOperand::Identifier(identifier) => {
                            Element::Place(Place::new(identifier.name))
                        }
                        ExpressionOperand::Block(block) => {
                            Element::Value(self.evaluate_block_expression(block)?)
                        }
                        ExpressionOperand::Match(r#match) => {
                            Element::Value(self.evaluate_match_expression(r#match)?)
                        }
                        ExpressionOperand::Conditional(conditional) => {
                            Element::Value(self.evaluate_conditional_expression(conditional)?)
                        }
                        ExpressionOperand::Array(array) => {
                            Element::Value(self.evaluate_array_expression(array)?)
                        }
                        ExpressionOperand::Tuple(tuple) => {
                            Element::Value(self.evaluate_tuple_expression(tuple)?)
                        }
                        ExpressionOperand::Structure(structure) => {
                            Element::Value(self.evaluate_structure_expression(structure)?)
                        }
                    };
                    self.rpn_stack.push(element);
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, true)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let (place, value) = operand_1
                        .assign(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.scope()
                        .borrow_mut()
                        .update_value(&place, value)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(Element::Value(Value::Unit));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
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
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
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
                ExpressionObject::Operator(ExpressionOperator::And) => {
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
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
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
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
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
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
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
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
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
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
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
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
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
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
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
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
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
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
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
                ExpressionObject::Operator(ExpressionOperator::Division) => {
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
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
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
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
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
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
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
                ExpressionObject::Operator(ExpressionOperator::Not) => {
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
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, false)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(
                        operand_1
                            .index(operand_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, false)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.rpn_stack.push(
                        operand_1
                            .field(operand_2)
                            .map_err(|error| Error::Element(element.location, error))?,
                    );
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, false)
                        .map_err(|error| Error::Scope(element.location, error))?;

                    let (identifier_1, identifier_2) = operand_1
                        .path(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    let value = match self
                        .scope()
                        .borrow()
                        .get_item_type(&identifier_1)
                        .map_err(|error| Error::Scope(element.location, error))?
                    {
                        ScopeItem::Type => match self
                            .scope()
                            .borrow()
                            .resolve_type(&identifier_1)
                            .map_err(|error| {
                            Error::Scope(element.location, error)
                        })? {
                            TypeVariant::Enumeration { variants } => {
                                let literals = variants
                                    .iter()
                                    .map(|(_key, value)| value)
                                    .collect::<Vec<&IntegerLiteral>>();
                                let bitlength = semantic::infer_enough_bitlength(&literals)
                                    .map_err(|error| {
                                        Error::BitlengthInference(element.location, error)
                                    })?;
                                let literal =
                                    variants.get(&identifier_2).cloned().ok_or_else(|| {
                                        Error::EnumerationVariantNotExists(
                                            element.location,
                                            identifier_1,
                                            identifier_2,
                                        )
                                    })?;
                                let namespace = self.next_temp_namespace();
                                let namespace = self.system.namespace(|| namespace);
                                Value::new_integer_from_literal(namespace, literal, Some(bitlength))
                                    .map_err(ElementError::Value)
                                    .map_err(|error| Error::Element(element.location, error))?
                            }
                            type_variant => {
                                return Err(Error::PathOperatorExpectedEnum(
                                    element.location,
                                    identifier_1,
                                    type_variant,
                                ))
                            }
                        },
                        item_type => {
                            return Err(Error::PathOperatorExpectedNamespace(
                                element.location,
                                identifier_1,
                                item_type,
                            ))
                        }
                    };

                    self.rpn_stack.push(Element::Value(value));
                }
            }
        }

        match self.rpn_stack.pop() {
            Some(Element::Value(value)) => Ok(value),
            Some(Element::Place(place)) => self
                .scope()
                .borrow()
                .get_value(&place)
                .map_err(|error| Error::Scope(location, error)),
            Some(Element::Type(..)) => panic!("Type expressions cannot be the expression result"),
            None => panic!("Always contains an element"),
        }
    }

    fn evaluate_block_expression(&mut self, block: BlockExpression) -> Result<Value, Error> {
        log::trace!("Block expression       : {}", block);

        self.push_scope();
        for statement in block.statements.into_iter() {
            self.execute_statement(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            self.evaluate_expression(*expression)?
        } else {
            Value::Unit
        };
        self.pop_scope();

        Ok(result)
    }

    fn evaluate_conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Value, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let location = conditional.location;

        let condition_result = match self.evaluate_expression(*conditional.condition)? {
            Value::Boolean(boolean) => boolean,
            value => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    conditional.location,
                    value,
                ))
            }
        };

        let main_result = { self.evaluate_block_expression(conditional.main_block)? };

        let else_result = if let Some(else_if) = conditional.else_if {
            self.evaluate_conditional_expression(*else_if)?
        } else if let Some(else_block) = conditional.else_block {
            self.evaluate_block_expression(else_block)?
        } else {
            Value::Unit
        };

        if !main_result.has_the_same_type_as(&else_result) {
            return Err(Error::ConditionalBranchTypeMismatch(
                location,
                main_result,
                else_result,
            ));
        }

        Ok(if condition_result.is_true() {
            main_result
        } else {
            else_result
        })
    }

    fn evaluate_match_expression(&mut self, r#match: MatchExpression) -> Result<Value, Error> {
        log::trace!("Match expression       : {}", r#match);

        let location = r#match.location;

        let match_expression_result = self.evaluate_expression(r#match.match_expression)?;

        for (left, right) in r#match.branches.into_iter() {
            let left = self.evaluate_expression(left)?;
            let namespace = self.next_temp_namespace();
            let namespace = self.system.namespace(|| namespace);

            let matched = match match_expression_result
                .equals(namespace, &left)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?
            {
                Value::Boolean(boolean) => boolean.is_true(),
                _ => panic!("Always is a boolean value"),
            };
            if matched {
                let right = self.evaluate_expression(right)?;
                return Ok(right);
            }
        }

        Ok(Value::new_unit())
    }

    fn evaluate_array_expression(&mut self, array: ArrayExpression) -> Result<Value, Error> {
        log::trace!("Array expression       : {}", array);

        let location = array.location;

        let mut values = Vec::with_capacity(array.elements.len());
        for element in array.elements.into_iter() {
            values.push(self.evaluate_expression(element)?);
        }

        Value::new_array(values)
            .map_err(ElementError::Value)
            .map_err(|error| Error::Element(location, error))
    }

    fn evaluate_tuple_expression(&mut self, tuple: TupleExpression) -> Result<Value, Error> {
        log::trace!("Tuple expression       : {}", tuple);

        let location = tuple.location;

        let mut values = Vec::with_capacity(tuple.elements.len());
        for element in tuple.elements.into_iter() {
            values.push(self.evaluate_expression(element)?);
        }

        Value::new_tuple(values)
            .map_err(ElementError::Value)
            .map_err(|error| Error::Element(location, error))
    }

    fn evaluate_structure_expression(
        &mut self,
        structure: StructureExpression,
    ) -> Result<Value, Error> {
        log::trace!("Structure expression       : {}", structure);

        let location = structure.location;

        let mut fields = Vec::with_capacity(structure.fields.len());
        for (identifier, expression) in structure.fields.into_iter() {
            fields.push((identifier.name, self.evaluate_expression(expression)?));
        }

        Value::new_structure(fields)
            .map_err(ElementError::Value)
            .map_err(|error| Error::Element(location, error))
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
                    self.scope().borrow().get_value(place).map(Element::Value)
                }
                Element::Value(value) => Ok(Element::Value(value)),
                Element::Type(..) => panic!("Type expressions cannot be resolved"),
            }
        } else {
            Ok(operand)
        }
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

    fn next_temp_namespace(&mut self) -> String {
        self.id_sequence += 1;
        format!("temp_{0:06}", self.id_sequence)
    }
}
