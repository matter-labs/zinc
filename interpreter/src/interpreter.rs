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
use parser::Location;
use parser::MatchExpression;
use parser::PatternVariant;
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
    pub fn new(mut scope: Scope) -> Self {
        let mut scope_stack = Vec::with_capacity(16);
        scope
            .declare_type(
                "require".to_owned(),
                TypeVariant::new_function(
                    vec![("condition".to_owned(), TypeVariant::new_boolean())],
                    TypeVariant::new_unit(),
                    BlockExpression::new(Location::new(0, 0), vec![], None),
                ),
            )
            .expect("Built-it function 'require' declaration error");
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
                                let location = r#type.location;
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
                                        r#type.location,
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
                        .map_err(|error| Error::Semantic(location, error))?;

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
                                    .expect("Always valid"),
                            ),
                            false,
                        )
                        .map_err(|error| Error::Scope(location, error))?;

                    if let Some(while_condition) = r#loop.while_condition.clone() {
                        let location = while_condition.location;
                        match self.evaluate_expression(while_condition)? {
                            Value::Boolean(boolean) => {
                                if boolean.is_false() {
                                    self.pop_scope();
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
                        .map(|field| (field.identifier.name, field.r#type.variant))
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
                        .map(|variant| (variant.identifier.name, variant.literal))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#enum.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Fn(r#fn) => {
                let location = r#fn.location;
                let type_variant = TypeVariant::new_function(
                    r#fn.arguments
                        .into_iter()
                        .map(|field| (field.identifier.name, field.r#type.variant))
                        .collect(),
                    r#fn.return_type.variant,
                    r#fn.body,
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#fn.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            Statement::Mod(_mod) => {}
            Statement::Use(_use) => {}
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
                        ExpressionOperand::Literal(literal) => {
                            Element::Value(self.evaluate_literal(literal)?)
                        }
                        ExpressionOperand::Type(r#type) => Element::Type(r#type.variant),
                        ExpressionOperand::Identifier(identifier) => {
                            if let Ok(type_variant @ TypeVariant::Function { .. }) =
                                self.scope().borrow_mut().resolve_type(&identifier.name)
                            {
                                Element::Type(type_variant)
                            } else {
                                Element::Place(Place::new(identifier.name))
                            }
                        }
                        ExpressionOperand::Block(block) => {
                            self.push_scope();
                            let value = self.evaluate_block_expression(block)?;
                            self.pop_scope();
                            Element::Value(value)
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
                        ExpressionOperand::List(list) => {
                            Element::Value(self.evaluate_list_expression(list)?)
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
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(false, false)
                        .map_err(|error| Error::Scope(element.location, error))?;

                    let type_variant = match operand_1 {
                        Element::Type(type_variant) => type_variant,
                        element => return Err(Error::CallingNotCallable(element.to_string())),
                    };
                    let arguments = match operand_2 {
                        Element::Value(Value::List(arguments)) => arguments,
                        _ => panic!("Always is an argument list"),
                    };

                    let value = self.evaluate_function_call(type_variant, arguments)?;
                    self.rpn_stack.push(Element::Value(value));
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
                                    .map_err(|error| Error::Semantic(element.location, error))?;
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
                            _ => panic!("Always is an enumeration or function"),
                        },
                        _ => panic!("Always is a type item"),
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

    fn evaluate_literal(&mut self, literal: Literal) -> Result<Value, Error> {
        log::trace!("Literal                : {}", literal);

        let location = literal.location;

        let result = match literal.data {
            InnerLiteral::Boolean(literal) => {
                let namespace = self.next_temp_namespace();
                let namespace = self.system.namespace(|| namespace);
                Value::new_boolean_from_literal(namespace, literal)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(location, error))?
            }
            InnerLiteral::Integer(literal) => {
                let namespace = self.next_temp_namespace();
                let namespace = self.system.namespace(|| namespace);
                Value::new_integer_from_literal(namespace, literal, None)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(location, error))?
            }
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        };

        Ok(result)
    }

    fn evaluate_block_expression(&mut self, block: BlockExpression) -> Result<Value, Error> {
        log::trace!("Block expression       : {}", block);

        for statement in block.statements.into_iter() {
            self.execute_statement(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            self.evaluate_expression(*expression)?
        } else {
            Value::Unit
        };

        Ok(result)
    }

    fn evaluate_conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Value, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let location = conditional.location;
        let condition_location = conditional.condition.location;

        let condition_result = match self.evaluate_expression(*conditional.condition)? {
            Value::Boolean(boolean) => boolean,
            value => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    condition_location,
                    value,
                ))
            }
        };

        self.push_scope();
        let main_result = self.evaluate_block_expression(conditional.main_block)?;
        self.pop_scope();

        let else_result = if let Some(else_if) = conditional.else_if {
            self.evaluate_conditional_expression(*else_if)?
        } else if let Some(else_block) = conditional.else_block {
            self.push_scope();
            let result = self.evaluate_block_expression(else_block)?;
            self.pop_scope();
            result
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

        for (pattern, expression) in r#match.branches.into_iter() {
            match pattern.variant {
                PatternVariant::Literal(literal) => {
                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    let literal_result = Value::new_from_literal(namespace, literal)
                        .map_err(ElementError::Value)
                        .map_err(|error| Error::Element(location, error))?;

                    let namespace = self.next_temp_namespace();
                    let namespace = self.system.namespace(|| namespace);
                    let matched = match literal_result
                        .equals(namespace, &match_expression_result)
                        .map_err(ElementError::Value)
                        .map_err(|error| Error::Element(location, error))?
                    {
                        Value::Boolean(boolean) => boolean.is_true(),
                        _ => panic!("Always is a boolean value"),
                    };

                    if matched {
                        self.push_scope();
                        let result = self.evaluate_expression(expression)?;
                        self.pop_scope();

                        return Ok(result);
                    }
                }
                PatternVariant::Binding(identifier) => {
                    self.scope()
                        .borrow_mut()
                        .declare_variable(identifier.name, match_expression_result, false)
                        .map_err(|error| Error::Scope(location, error))?;

                    self.push_scope();
                    let result = self.evaluate_expression(expression)?;
                    self.pop_scope();

                    return Ok(result);
                }
                PatternVariant::Ignoring => {
                    self.push_scope();
                    let result = self.evaluate_expression(expression)?;
                    self.pop_scope();

                    return Ok(result);
                }
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
        log::trace!("Structure expression   : {}", structure);

        let location = structure.location;

        let mut fields = Vec::with_capacity(structure.fields.len());
        for (identifier, expression) in structure.fields.into_iter() {
            fields.push((identifier.name, self.evaluate_expression(expression)?));
        }

        Value::new_structure(fields)
            .map_err(ElementError::Value)
            .map_err(|error| Error::Element(location, error))
    }

    fn evaluate_list_expression(&mut self, list: Vec<Expression>) -> Result<Value, Error> {
        log::trace!("List expression        : {:?}", list);

        let mut values = Vec::with_capacity(list.len());
        for expression in list.into_iter() {
            values.push(self.evaluate_expression(expression)?);
        }

        Ok(Value::List(values))
    }

    fn evaluate_function_call(
        &mut self,
        function: TypeVariant,
        mut arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        let global_scope = self.scope_stack[0].clone();
        let scope = Rc::new(RefCell::new(Scope::new(Some(global_scope.clone()))));
        let new_scope_stack = vec![global_scope, scope];

        match function {
            TypeVariant::Function {
                arguments: argument_fields,
                return_type,
                body,
            } => {
                arguments.reverse();
                let mut data: Vec<(String, TypeVariant, Value)> =
                    Vec::with_capacity(argument_fields.len());
                for (name, type_variant) in argument_fields.into_iter() {
                    let value = arguments
                        .pop()
                        .ok_or_else(|| Error::MissingFunctionArgument(name.clone()))?;
                    let value_type_variant = value.type_variant();
                    if value_type_variant != type_variant {
                        return Err(Error::FunctionArgumentTypeMismatch(
                            type_variant,
                            value_type_variant,
                        ));
                    }
                    data.push((name, type_variant, value));
                }
                for argument in data.into_iter() {
                    new_scope_stack[1]
                        .borrow_mut()
                        .declare_variable(argument.0, argument.2, false)
                        .map_err(|error| Error::Scope(body.location, error))?;
                }

                let old_scope_stack = self.scope_stack.drain(..).collect();
                self.scope_stack = new_scope_stack;
                let result = self.evaluate_block_expression(body)?;
                let return_type = *return_type;
                let result_type_variant = result.type_variant();
                self.scope_stack = old_scope_stack;
                if return_type != result_type_variant {
                    Err(Error::FunctionReturnTypeMismatch(
                        return_type,
                        result_type_variant,
                    ))
                } else {
                    Ok(result)
                }
            }
            type_variant => Err(Error::CallingNotCallable(type_variant.to_string())),
        }
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
