//!
//! The expression semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operand::array::builder::Builder as GeneratorArrayExpressionBuilder;
use crate::generator::expression::operand::array::Expression as GeneratorArrayExpression;
use crate::generator::expression::operand::block::builder::Builder as GeneratorBlockExpressionBuilder;
use crate::generator::expression::operand::block::Expression as GeneratorBlockExpression;
use crate::generator::expression::operand::conditional::builder::Builder as GeneratorConditionalExpressionBuilder;
use crate::generator::expression::operand::conditional::Expression as GeneratorConditionalExpression;
use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::group::Expression as GeneratorGroupExpression;
use crate::generator::expression::operand::r#match::builder::Builder as GeneratorMatchExpressionBuilder;
use crate::generator::expression::operand::r#match::Expression as GeneratorMatchExpression;
use crate::generator::expression::operand::variable::Variable as GeneratorVariable;
use crate::generator::expression::operand::Operand as GeneratorOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::analyzer::translation_hint::TranslationHint;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::builtin::Function as BuiltInFunctionType;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::Array;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::Structure;
use crate::semantic::element::value::tuple::Tuple;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Variant as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax;
use crate::syntax::ArrayExpression;
use crate::syntax::ArrayExpressionVariant;
use crate::syntax::BlockExpression;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionAuxiliary;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpression;
use crate::syntax::MatchPatternVariant;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    intermediate: GeneratorExpression,
    is_next_call_builtin: bool,
}

#[derive(Debug, Clone)]
enum StackElement {
    NotEvaluated(ExpressionOperand),
    Evaluated(Element),
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;

    pub fn new(scope: Rc<RefCell<Scope>>) -> Self {
        Self {
            scope_stack: {
                let mut scope_stack = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scope_stack.push(scope);
                scope_stack
            },
            operands: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
            intermediate: GeneratorExpression::new(),
            is_next_call_builtin: false,
        }
    }

    pub fn expression(
        &mut self,
        expression: Expression,
        translation_hint: TranslationHint,
    ) -> Result<(Element, GeneratorExpression), Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Assignment);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseOr) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_bitwise_or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentBitwiseOr);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseXor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_bitwise_xor(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentBitwiseXor);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseAnd) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_bitwise_and(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentBitwiseAnd);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseShiftLeft) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_bitwise_shift_left(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentBitwiseShiftLeft);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseShiftRight) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_bitwise_shift_right(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentBitwiseShiftRight);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentAddition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_add(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentAddition);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentSubtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_subtract(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentSubtraction);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentMultiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_multiply(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentMultiplication);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentDivision) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_divide(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentDivision);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentRemainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope())?;
                    let place = operand_1
                        .assign_remainder(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory {
                            location: element.location,
                            name: place.to_string(),
                            reference: item_location,
                        });
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType {
                            location: element.location,
                            expected: r#type.to_string(),
                            found: place.r#type.to_string(),
                        });
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::AssignmentRemainder);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .range_inclusive(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .range(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Or);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .xor(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Xor);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .and(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::And);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .equals(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Equals);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .not_equals(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::NotEquals);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .greater_equals(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::GreaterEquals);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .lesser_equals(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::LesserEquals);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .greater(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Greater);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .lesser(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Lesser);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseOr) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .bitwise_or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseOr);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseXor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .bitwise_xor(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseXor);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseAnd) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .bitwise_and(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseAnd);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseShiftLeft) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .bitwise_shift_left(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseShiftLeft);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseShiftRight) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .bitwise_shift_right(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseShiftRight);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .add(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Addition);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .subtract(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Subtraction);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .multiply(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Multiplication);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .divide(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Division);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .remainder(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Remainder);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::TypeExpression,
                    )?;

                    if let Element::Type(ref r#type) = operand_2 {
                        if let Some(operator) = GeneratorExpressionOperator::casting(r#type) {
                            self.intermediate.push_operator(operator);
                        }
                    }
                    let result = operand_1
                        .cast(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Not);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseNot) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .bitwise_not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseNot);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Negation);
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Index) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let (result, access) = operand_1
                        .index(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::ArrayIndex);

                    match result {
                        operand @ Element::Place(_) => {
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            let value = Value::try_from(&access.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.push_operand(StackElement::Evaluated(Element::Value(value)));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::CompoundTypeMember,
                    )?;

                    let (result, access) = operand_1
                        .field(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Slice);

                    match result {
                        operand @ Element::Place(_) => {
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            let value = Value::try_from(&access.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.push_operand(StackElement::Evaluated(Element::Value(value)));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    self.operator_call(element)?
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PathExpression,
                        TranslationHint::CompoundTypeMember,
                    )?;

                    let result = operand_1
                        .path(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::CallBuiltIn) => {
                    self.is_next_call_builtin = true;
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd) => {
                    let element = self
                        .evaluate_operand(TranslationHint::ValueExpression)
                        .map(StackElement::Evaluated)?;
                    self.push_operand(element);
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceCopy) => {}
            }
        }

        Ok((
            self.evaluate_operand(translation_hint)?,
            self.intermediate.clone(),
        ))
    }

    pub fn operator_call(&mut self, element: ExpressionElement) -> Result<(), Error> {
        let location = element.location;
        let is_next_call_builtin = self.is_next_call_builtin;
        self.is_next_call_builtin = false;

        let (operand_1, operand_2) = self.evaluate_binary_operands(
            TranslationHint::TypeExpression,
            TranslationHint::ValueExpression,
        )?;

        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => match Scope::resolve_path(self.scope(), &path)?.variant {
                ScopeItem::Type(Type::Function(function)) => function,
                item => {
                    return Err(Error::Function(
                        element.location,
                        FunctionError::non_callable(item.to_string()),
                    ));
                }
            },
            operand => {
                return Err(Error::Function(
                    element.location,
                    FunctionError::non_callable(operand.to_string()),
                ));
            }
        };

        let argument_elements = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        let mut input_size = 0;
        for element in argument_elements.iter() {
            input_size += Type::from_element(element, self.scope())?.size();
        }

        let return_type =
            match function {
                FunctionType::UserDefined(function) => {
                    if is_next_call_builtin {
                        return Err(Error::Function(
                            element.location,
                            FunctionError::BuiltIn(BuiltInFunctionError::unknown(
                                function.identifier().to_owned(),
                            )),
                        ));
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::call(input_size));

                    function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?
                }
                FunctionType::BuiltInFunction(function) => {
                    if !is_next_call_builtin {
                        return Err(Error::Function(
                            element.location,
                            FunctionError::BuiltIn(BuiltInFunctionError::specifier_missing(
                                function.identifier(),
                            )),
                        ));
                    }

                    match function {
                        BuiltInFunctionType::Debug(function) => {
                            let (return_type, format, argument_types) = function
                                .call(argument_elements)
                                .map_err(|error| Error::Function(element.location, error))?;

                            self.intermediate.push_operator(
                                GeneratorExpressionOperator::call_debug(format, argument_types),
                            );

                            return_type
                        }
                        BuiltInFunctionType::Assert(function) => {
                            let (return_type, message) = function
                                .call(argument_elements)
                                .map_err(|error| Error::Function(element.location, error))?;

                            self.intermediate
                                .push_operator(GeneratorExpressionOperator::call_assert(message));

                            return_type
                        }
                    }
                }
                FunctionType::StandardLibrary(function) => {
                    if is_next_call_builtin {
                        return Err(Error::Function(
                            element.location,
                            FunctionError::BuiltIn(BuiltInFunctionError::unknown(
                                function.identifier().to_owned(),
                            )),
                        ));
                    }

                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::call_std(
                            function.builtin_identifier(),
                        ));

                    function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?
                }
            };

        self.push_operand(StackElement::Evaluated(Element::Value(
            Value::try_from(&return_type)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
        )));
        Ok(())
    }

    pub fn block_expression(
        &mut self,
        block: BlockExpression,
    ) -> Result<(Element, GeneratorBlockExpression), Error> {
        let mut builder = GeneratorBlockExpressionBuilder::default();

        for statement in block.statements.into_iter() {
            if let Some(statement) = StatementAnalyzer::new(self.scope(), HashMap::new())
                .function_local_statement(statement)?
            {
                builder.push_statement(statement);
            }
        }

        let result = match block.expression {
            Some(expression) => {
                let (element, expression) =
                    self.expression(*expression, TranslationHint::ValueExpression)?;
                builder.set_expression(expression);
                element
            }
            None => Element::Value(Value::Unit),
        };

        Ok((result, builder.finish()))
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let constant = Constant::from(literal);

        if let Some(constant) = GeneratorConstant::try_from_semantic(&constant) {
            self.intermediate
                .push_operand(GeneratorOperand::Constant(constant));
        }

        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let constant = IntegerConstant::try_from(&literal)
            .map(Constant::Integer)
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;

        if let Some(constant) = GeneratorConstant::try_from_semantic(&constant) {
            self.intermediate
                .push_operand(GeneratorOperand::Constant(constant));
        }

        Ok(Element::Constant(constant))
    }

    fn string_literal(&mut self, literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }

    fn member_integer(&mut self, integer: MemberInteger) -> Result<Element, Error> {
        let location = integer.location;

        let integer = IntegerConstant::try_from(&integer.literal)
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?
            .to_usize()
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;

        Ok(Element::MemberInteger(integer))
    }

    fn member_string(&mut self, member_string: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_string))
    }

    fn identifier(
        &mut self,
        identifier: Identifier,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = identifier.location;

        let path = Path::new(location, identifier.into());

        self.translate_path(&path, translation_hint)
    }

    fn r#type(&mut self, r#type: syntax::Type) -> Result<Element, Error> {
        Type::from_type_variant(&r#type.variant, self.scope()).map(Element::Type)
    }

    fn conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<(Element, GeneratorConditionalExpression), Error> {
        let condition_location = conditional.condition.location;
        let main_expression_location = conditional
            .main_block
            .expression
            .as_ref()
            .map(|expression| expression.location)
            .unwrap_or(conditional.main_block.location);
        let else_expression_location = conditional
            .else_block
            .as_ref()
            .map(|block| {
                block
                    .expression
                    .as_ref()
                    .map(|expression| expression.location)
                    .unwrap_or(block.location)
            })
            .unwrap_or(conditional.location);

        let mut builder = GeneratorConditionalExpressionBuilder::default();

        let (condition_result, condition) =
            self.expression(*conditional.condition, TranslationHint::ValueExpression)?;
        match Type::from_element(&condition_result, self.scope())? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition {
                    location: condition_location,
                    found: r#type.to_string(),
                });
            }
        }
        builder.set_condition(condition);

        self.push_scope();
        let (main_result, main_block) = self.block_expression(conditional.main_block)?;
        let main_type = Type::from_element(&main_result, self.scope())?;
        self.pop_scope();
        builder.set_main_block(main_block);

        let else_type = if let Some(else_block) = conditional.else_block {
            self.push_scope();
            let (else_result, else_block) = self.block_expression(else_block)?;
            let else_type = Type::from_element(&else_result, self.scope())?;
            self.pop_scope();
            builder.set_else_block(else_block);

            else_type
        } else {
            Type::Unit
        };

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch {
                location: main_expression_location,
                expected: main_type.to_string(),
                found: else_type.to_string(),
                reference: else_expression_location,
            });
        }

        Ok((main_result, builder.finish()))
    }

    fn match_expression(
        &mut self,
        r#match: MatchExpression,
    ) -> Result<(Element, GeneratorMatchExpression), Error> {
        let location = r#match.location;

        let mut builder = GeneratorMatchExpressionBuilder::default();

        let scrutinee_location = r#match.scrutinee.location;
        let (scrutinee_result, intermediate_scrutinee) =
            self.expression(r#match.scrutinee, TranslationHint::ValueExpression)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, self.scope())?;
        builder.set_scrutinee(intermediate_scrutinee);

        let first_branch_expression_location = r#match.branches[0].1.location;

        let mut is_exhausted = false;
        let mut branch_results = Vec::with_capacity(r#match.branches.len());

        if r#match.branches.len() < 2 {
            return Err(Error::MatchLessThanTwoBranches { location });
        }

        for (pattern, expression) in r#match.branches.into_iter() {
            let pattern_location = pattern.location;
            let expression_location = expression.location;

            if is_exhausted {
                return Err(Error::MatchBranchUnreachable {
                    location: pattern.location,
                });
            }
            let result = match pattern.variant {
                MatchPatternVariant::BooleanLiteral(boolean) => {
                    let constant = Constant::from(boolean);
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    let constant = GeneratorConstant::try_from_semantic(&constant)
                        .expect(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        self.expression(expression, TranslationHint::ValueExpression)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::IntegerLiteral(integer) => {
                    let location = integer.location;

                    let constant = IntegerConstant::try_from(&integer)
                        .map(Constant::Integer)
                        .map_err(|error| {
                            Error::Element(
                                location,
                                ElementError::Constant(ConstantError::Integer(error)),
                            )
                        })?;
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    let constant = GeneratorConstant::try_from_semantic(&constant)
                        .expect(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        self.expression(expression, TranslationHint::ValueExpression)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    let constant = match self.expression(path, TranslationHint::ValueExpression)? {
                        (Element::Constant(constant), _intermediate) => constant,
                        (element, _intermediate) => {
                            return Err(Error::MatchBranchPatternPathExpectedConstant {
                                location,
                                found: element.to_string(),
                            });
                        }
                    };

                    let constant = GeneratorConstant::try_from_semantic(&constant)
                        .expect(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        self.expression(expression, TranslationHint::ValueExpression)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    self.push_scope();
                    Scope::declare_variable(
                        self.scope(),
                        identifier,
                        ScopeVariableItem::new(false, scrutinee_type.clone()),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                    let (result, branch) =
                        self.expression(expression, TranslationHint::ValueExpression)?;
                    builder.set_else_branch(branch);
                    self.pop_scope();

                    result
                }
                MatchPatternVariant::Wildcard => {
                    is_exhausted = true;
                    let (result, branch) =
                        self.expression(expression, TranslationHint::ValueExpression)?;
                    builder.set_else_branch(branch);
                    result
                }
            };

            let result_type = Type::from_element(&result, self.scope())?;
            if let Some(first_branch_result) = branch_results.get(0) {
                let first_branch_result_type =
                    Type::from_element(first_branch_result, self.scope())?;
                if result_type != first_branch_result_type {
                    return Err(Error::MatchBranchExpressionInvalidType {
                        location: expression_location,
                        expected: first_branch_result_type.to_string(),
                        found: result_type.to_string(),
                        reference: first_branch_expression_location,
                    });
                }
            }

            branch_results.push(result);
        }

        if !is_exhausted {
            return Err(Error::MatchNotExhausted { location });
        }

        let element = match branch_results.pop() {
            Some(result) => result,
            None => Element::Constant(Constant::Unit),
        };

        Ok((element, builder.finish()))
    }

    fn array_expression(
        &mut self,
        array: ArrayExpression,
    ) -> Result<(Element, GeneratorArrayExpression), Error> {
        let mut result = Array::default();
        let mut builder = GeneratorArrayExpressionBuilder::default();

        match array.variant {
            ArrayExpressionVariant::List { elements } => {
                for expression in elements.into_iter() {
                    let expression_location = expression.location;

                    let (element, expression) = Self::new(self.scope())
                        .expression(expression, TranslationHint::ValueExpression)?;
                    let element_type = Type::from_element(&element, self.scope())?;
                    result.push(element_type).map_err(|error| {
                        Error::Element(
                            expression_location,
                            ElementError::Value(ValueError::Array(error)),
                        )
                    })?;

                    builder.push_expression(expression);
                }
            }
            ArrayExpressionVariant::Repeated {
                expression,
                size_expression,
            } => {
                let expression_location = expression.location;
                let size_expression_location = size_expression.location;

                let size = match Self::new(self.scope())
                    .expression(size_expression, TranslationHint::ValueExpression)?
                {
                    (Element::Constant(Constant::Integer(integer)), _intermediate) => {
                        integer.to_usize().map_err(|error| {
                            Error::Element(
                                size_expression_location,
                                ElementError::Constant(ConstantError::Integer(error)),
                            )
                        })?
                    }
                    (element, _intermediate) => {
                        return Err(Error::ConstantExpressionHasNonConstantElement {
                            location: size_expression_location,
                            found: element.to_string(),
                        });
                    }
                };

                let (element, expression) =
                    self.expression(expression, TranslationHint::ValueExpression)?;
                let element_type = Type::from_element(&element, self.scope())?;
                result.extend(element_type, size).map_err(|error| {
                    Error::Element(
                        expression_location,
                        ElementError::Value(ValueError::Array(error)),
                    )
                })?;

                builder.push_expression(expression);
                builder.set_size(size);
            }
        }

        let result = Element::Value(Value::Array(result));

        Ok((result, builder.finish()))
    }

    fn tuple_expression(
        &mut self,
        tuple: TupleExpression,
    ) -> Result<(Element, GeneratorGroupExpression), Error> {
        let mut result = Tuple::default();
        let mut builder = GeneratorGroupExpressionBuilder::default();

        for expression in tuple.elements.into_iter() {
            let (element, expression) =
                self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result.push(element_type);

            builder.push_expression(expression);
        }

        let result = Element::Value(Value::Tuple(result));

        Ok((result, builder.finish()))
    }

    fn structure_expression(
        &mut self,
        structure: StructureExpression,
    ) -> Result<(Element, GeneratorGroupExpression), Error> {
        let identifier_location = structure.identifier.location;

        let mut builder = GeneratorGroupExpressionBuilder::default();

        let structure_type = match Scope::resolve_item(self.scope(), &structure.identifier.name)
            .map_err(|error| Error::Scope(identifier_location, error))?
            .variant
        {
            ScopeItem::Type(Type::Structure(structure)) => structure,
            item => {
                return Err(Error::TypeAliasDoesNotPointToStructure {
                    location: identifier_location,
                    found: item.to_string(),
                });
            }
        };
        let mut result = Structure::new(structure_type);

        for (identifier, expression) in structure.fields.into_iter() {
            let identifier_location = identifier.location;

            let (element, expression) =
                self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result
                .push(identifier.name.clone(), element_type)
                .map_err(|error| {
                    Error::Element(
                        identifier_location,
                        ElementError::Value(ValueError::Structure(error)),
                    )
                })?;

            builder.push_expression(expression);
        }

        let result = Element::Value(Value::Structure(result));

        Ok((result, builder.finish()))
    }

    fn list_expression(
        &mut self,
        list: Vec<Expression>,
    ) -> Result<(Element, GeneratorGroupExpression), Error> {
        let mut elements = Vec::with_capacity(list.len());
        let mut builder = GeneratorGroupExpressionBuilder::default();

        for expression in list.into_iter() {
            let (element, expression) =
                self.expression(expression, TranslationHint::ValueExpression)?;
            elements.push(element);

            builder.push_expression(expression);
        }

        let result = Element::ArgumentList(elements);

        Ok((result, builder.finish()))
    }

    fn translate_path(
        &mut self,
        path: &Path,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = path.location;

        let path_last = path.last();

        match translation_hint {
            TranslationHint::PlaceExpression => {
                let identifier = path.last().name.to_owned();
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Variable(variable) => Ok(Element::Place(Place::new(
                        location,
                        identifier,
                        variable.r#type,
                        variable.is_mutable,
                    ))),
                    ScopeItem::Constant(constant) => Ok(Element::Constant(constant)),
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
                }
            }
            TranslationHint::ValueExpression => {
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Variable(variable) => {
                        let value = Value::try_from(&variable.r#type)
                            .map_err(ElementError::Value)
                            .map_err(|error| Error::Element(location, error))?;
                        if let Some(variable) = GeneratorVariable::try_from_semantic(&value) {
                            self.intermediate
                                .push_operand(GeneratorOperand::Variable(variable));
                        }
                        Ok(Element::Value(value))
                    }
                    ScopeItem::Constant(constant) => {
                        if let Some(constant) = GeneratorConstant::try_from_semantic(&constant) {
                            self.intermediate
                                .push_operand(GeneratorOperand::Constant(constant));
                        }
                        Ok(Element::Constant(constant))
                    }
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
                }
            }

            TranslationHint::TypeExpression => {
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    _ => Ok(Element::Path(path.to_owned())),
                }
            }
            TranslationHint::PathExpression => Ok(Element::Path(path.to_owned())),
            TranslationHint::CompoundTypeMember => Ok(Element::MemberString(MemberString::new(
                location,
                path_last.name.to_owned(),
            ))),
        }
    }

    fn translate_place(
        &mut self,
        place: &Place,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        match translation_hint {
            TranslationHint::ValueExpression => Value::try_from(&place.r#type)
                .map(Element::Value)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(place.location, error)),
            _ => Ok(Element::Place(place.to_owned())),
        }
    }

    fn evaluate_operand(&mut self, translation_hint: TranslationHint) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Constant(Constant::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => self.boolean_literal(literal),
                ExpressionOperand::LiteralInteger(literal) => self.integer_literal(literal),
                ExpressionOperand::LiteralString(literal) => self.string_literal(literal),
                ExpressionOperand::MemberInteger(integer) => self.member_integer(integer),
                ExpressionOperand::MemberString(identifier) => self.member_string(identifier),
                ExpressionOperand::Identifier(identifier) => {
                    self.identifier(identifier, translation_hint)
                }
                ExpressionOperand::Type(r#type) => self.r#type(r#type),
                ExpressionOperand::List(expressions) => {
                    let (result, intermediate) = self.list_expression(expressions)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Block(expression) => {
                    self.push_scope();
                    let (result, intermediate) = self.block_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Block(intermediate));
                    self.pop_scope();
                    Ok(result)
                }
                ExpressionOperand::Conditional(expression) => {
                    let (result, intermediate) = self.conditional_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Conditional(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Match(expression) => {
                    let (result, intermediate) = self.match_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Match(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Array(expression) => {
                    let (result, intermediate) = self.array_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Array(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Tuple(expression) => {
                    let (result, intermediate) = self.tuple_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Structure(expression) => {
                    let (result, intermediate) = self.structure_expression(expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
            },
            StackElement::Evaluated(element) => match element {
                Element::Path(path) => self.translate_path(&path, translation_hint),
                Element::Place(place) => self.translate_place(&place, translation_hint),
                element => Ok(element),
            },
        }
    }

    fn evaluate_unary_operand(
        &mut self,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        self.evaluate_operand(translation_hint)
    }

    fn evaluate_binary_operands(
        &mut self,
        translation_hint_1: TranslationHint,
        translation_hint_2: TranslationHint,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand(translation_hint_2)?;
        let operand_1 = self.evaluate_operand(translation_hint_1)?;
        Ok((operand_1, operand_2))
    }

    fn push_operand(&mut self, operand: StackElement) {
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(Scope::new_child(self.scope()));
    }

    fn pop_scope(&mut self) {
        self.scope_stack
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }
}
