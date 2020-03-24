//!
//! The expression semantic analyzer.
//!

pub mod array;
pub mod block;
pub mod hint;
pub mod identifier;
pub mod list;
pub mod literal;
pub mod member;
pub mod path;
pub mod place;
pub mod stack;
pub mod structure;
pub mod tuple;
pub mod r#type;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operand::conditional::builder::Builder as GeneratorConditionalExpressionBuilder;
use crate::generator::expression::operand::conditional::Expression as GeneratorConditionalExpression;
use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::r#match::builder::Builder as GeneratorMatchExpressionBuilder;
use crate::generator::expression::operand::r#match::Expression as GeneratorMatchExpression;
use crate::generator::expression::operand::Operand as GeneratorOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::builtin::Function as BuiltInFunctionType;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Variant as ScopeItem;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionAuxiliary;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::MatchExpression;
use crate::syntax::MatchPatternVariant;

use self::array::Analyzer as ArrayAnalyzer;
use self::block::Analyzer as BlockAnalyzer;
use self::hint::Hint as TranslationHint;
use self::identifier::Analyzer as IdentifierAnalyzer;
use self::list::Analyzer as ListAnalyzer;
use self::literal::Analyzer as LiteralAnalyzer;
use self::member::Analyzer as MemberAnalyzer;
use self::path::Translator as PathTranslator;
use self::place::Translator as PlaceTranslator;
use self::r#type::Analyzer as TypeAnalyzer;
use self::stack::element::Element as StackElement;
use self::stack::Stack as EvaluationStack;
use self::structure::Analyzer as StructureAnalyzer;
use self::tuple::Analyzer as TupleAnalyzer;

pub struct Analyzer {
    scope_stack: ScopeStack,
    evaluation_stack: EvaluationStack,
    intermediate: GeneratorExpression,
    is_next_call_builtin: bool,
}

impl Analyzer {
    pub fn new(scope: Rc<RefCell<Scope>>) -> Self {
        Self {
            scope_stack: ScopeStack::new(scope),
            evaluation_stack: EvaluationStack::default(),
            intermediate: GeneratorExpression::default(),
            is_next_call_builtin: false,
        }
    }

    pub fn analyze(
        mut self,
        expression: Expression,
        translation_hint: TranslationHint,
    ) -> Result<(Element, GeneratorExpression), Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => self
                    .evaluation_stack
                    .push(StackElement::NotEvaluated(operand)),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseOr) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_bitwise_or(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseXor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_bitwise_xor(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseAnd) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_bitwise_and(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseShiftLeft) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_bitwise_shift_left(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentBitwiseShiftRight) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_bitwise_shift_right(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentAddition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_add(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentSubtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_subtract(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentMultiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_multiply(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentDivision) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_divide(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentRemainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
                    let place = operand_1
                        .assign_remainder(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
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
                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .range_inclusive(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.evaluation_stack.push(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let result = operand_1
                        .range(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Not);
                    self.evaluation_stack.push(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::BitwiseNot) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .bitwise_not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseNot);
                    self.evaluation_stack.push(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Negation);
                    self.evaluation_stack.push(StackElement::Evaluated(result));
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
                            self.evaluation_stack.push(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            let value = Value::try_from(&access.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.evaluation_stack
                                .push(StackElement::Evaluated(Element::Value(value)));
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
                            self.evaluation_stack.push(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            let value = Value::try_from(&access.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.evaluation_stack
                                .push(StackElement::Evaluated(Element::Value(value)));
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
                    self.evaluation_stack.push(StackElement::Evaluated(result));
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::CallBuiltIn) => {
                    self.is_next_call_builtin = true;
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd) => {
                    let element = self
                        .evaluate_operand(TranslationHint::ValueExpression)
                        .map(StackElement::Evaluated)?;
                    self.evaluation_stack.push(element);
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceCopy) => {}
            }
        }

        Ok((self.evaluate_operand(translation_hint)?, self.intermediate))
    }

    fn operator_call(&mut self, element: ExpressionElement) -> Result<(), Error> {
        let location = element.location;
        let is_next_call_builtin = self.is_next_call_builtin;
        self.is_next_call_builtin = false;

        let (operand_1, operand_2) = self.evaluate_binary_operands(
            TranslationHint::TypeExpression,
            TranslationHint::ValueExpression,
        )?;

        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => {
                match Scope::resolve_path(self.scope_stack.top(), &path)?.variant {
                    ScopeItem::Type(Type::Function(function)) => function,
                    item => {
                        return Err(Error::Function(
                            element.location,
                            FunctionError::non_callable(item.to_string()),
                        ));
                    }
                }
            }
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
            input_size += Type::from_element(element, self.scope_stack.top())?.size();
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

        self.evaluation_stack
            .push(StackElement::Evaluated(Element::Value(
                Value::try_from(&return_type)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(location, error))?,
            )));
        Ok(())
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

        let (condition_result, condition) = Self::new(self.scope_stack.top())
            .analyze(*conditional.condition, TranslationHint::ValueExpression)?;
        match Type::from_element(&condition_result, self.scope_stack.top())? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition {
                    location: condition_location,
                    found: r#type.to_string(),
                });
            }
        }
        builder.set_condition(condition);

        self.scope_stack.push();
        let (main_result, main_block) =
            BlockAnalyzer::analyze(self.scope_stack.top(), conditional.main_block)?;
        let main_type = Type::from_element(&main_result, self.scope_stack.top())?;
        self.scope_stack.pop();
        builder.set_main_block(main_block);

        let else_type = if let Some(else_block) = conditional.else_block {
            self.scope_stack.push();
            let (else_result, else_block) =
                BlockAnalyzer::analyze(self.scope_stack.top(), else_block)?;
            let else_type = Type::from_element(&else_result, self.scope_stack.top())?;
            self.scope_stack.pop();
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
        let (scrutinee_result, intermediate_scrutinee) = Self::new(self.scope_stack.top())
            .analyze(r#match.scrutinee, TranslationHint::ValueExpression)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, self.scope_stack.top())?;
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
                    let (result, branch) = Self::new(self.scope_stack.top())
                        .analyze(expression, TranslationHint::ValueExpression)?;
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
                    let (result, branch) = Self::new(self.scope_stack.top())
                        .analyze(expression, TranslationHint::ValueExpression)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    let constant = match Self::new(self.scope_stack.top())
                        .analyze(path, TranslationHint::ValueExpression)?
                    {
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
                    let (result, branch) = Self::new(self.scope_stack.top())
                        .analyze(expression, TranslationHint::ValueExpression)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    self.scope_stack.push();
                    Scope::declare_variable(
                        self.scope_stack.top(),
                        identifier,
                        ScopeVariableItem::new(false, scrutinee_type.clone()),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                    let (result, branch) = Self::new(self.scope_stack.top())
                        .analyze(expression, TranslationHint::ValueExpression)?;
                    builder.set_else_branch(branch);
                    self.scope_stack.pop();

                    result
                }
                MatchPatternVariant::Wildcard => {
                    is_exhausted = true;
                    let (result, branch) = Self::new(self.scope_stack.top())
                        .analyze(expression, TranslationHint::ValueExpression)?;
                    builder.set_else_branch(branch);
                    result
                }
            };

            let result_type = Type::from_element(&result, self.scope_stack.top())?;
            if let Some(first_branch_result) = branch_results.get(0) {
                let first_branch_result_type =
                    Type::from_element(first_branch_result, self.scope_stack.top())?;
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

    fn evaluate_operand(&mut self, translation_hint: TranslationHint) -> Result<Element, Error> {
        match self.evaluation_stack.pop() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Constant(Constant::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => {
                    let (result, intermediate) = LiteralAnalyzer::boolean(literal)?;
                    if let Some(intermediate) = intermediate {
                        self.intermediate
                            .push_operand(GeneratorOperand::Constant(intermediate));
                    }
                    Ok(result)
                }
                ExpressionOperand::LiteralInteger(literal) => {
                    let (result, intermediate) = LiteralAnalyzer::integer(literal)?;
                    if let Some(intermediate) = intermediate {
                        self.intermediate
                            .push_operand(GeneratorOperand::Constant(intermediate));
                    }
                    Ok(result)
                }
                ExpressionOperand::LiteralString(literal) => LiteralAnalyzer::string(literal),
                ExpressionOperand::MemberInteger(integer) => MemberAnalyzer::integer(integer),
                ExpressionOperand::MemberString(identifier) => MemberAnalyzer::string(identifier),
                ExpressionOperand::Identifier(identifier) => {
                    let (result, intermediate) = IdentifierAnalyzer::analyze(
                        self.scope_stack.top(),
                        identifier,
                        translation_hint,
                    )?;
                    if let Some(operand) = intermediate {
                        self.intermediate.push_operand(operand);
                    }
                    Ok(result)
                }
                ExpressionOperand::Type(r#type) => {
                    TypeAnalyzer::analyze(self.scope_stack.top(), r#type)
                }
                ExpressionOperand::Array(expression) => {
                    let (result, intermediate) =
                        ArrayAnalyzer::analyze(self.scope_stack.top(), expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Array(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Tuple(expression) => {
                    let (result, intermediate) =
                        TupleAnalyzer::analyze(self.scope_stack.top(), expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Structure(expression) => {
                    let (result, intermediate) =
                        StructureAnalyzer::analyze(self.scope_stack.top(), expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
                ExpressionOperand::List(expressions) => {
                    let (result, intermediate) =
                        ListAnalyzer::analyze(self.scope_stack.top(), expressions)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Group(intermediate));
                    Ok(result)
                }
                ExpressionOperand::Block(expression) => {
                    let (result, intermediate) =
                        BlockAnalyzer::analyze(self.scope_stack.top(), expression)?;
                    self.intermediate
                        .push_operand(GeneratorOperand::Block(intermediate));
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
            },
            StackElement::Evaluated(element) => match element {
                Element::Path(path) => {
                    let (element, intermediate) =
                        PathTranslator::translate(self.scope_stack.top(), &path, translation_hint)?;
                    if let Some(operand) = intermediate {
                        self.intermediate.push_operand(operand);
                    }
                    Ok(element)
                }
                Element::Place(place) => PlaceTranslator::translate(&place, translation_hint),
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
}
