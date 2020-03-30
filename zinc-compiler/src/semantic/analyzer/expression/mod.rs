//!
//! The expression semantic analyzer.
//!

mod tests;

pub mod array;
pub mod block;
pub mod call;
pub mod conditional;
pub mod hint;
pub mod identifier;
pub mod list;
pub mod literal;
pub mod r#match;
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

use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::ExpressionTree;
use crate::syntax::ExpressionTreeNode;

use self::array::Analyzer as ArrayAnalyzer;
use self::block::Analyzer as BlockAnalyzer;
use self::call::Analyzer as CallAnalyzer;
use self::conditional::Analyzer as ConditionalAnalyzer;
use self::hint::Hint as TranslationHint;
use self::identifier::Analyzer as IdentifierAnalyzer;
use self::list::Analyzer as ListAnalyzer;
use self::literal::Analyzer as LiteralAnalyzer;
use self::member::Analyzer as MemberAnalyzer;
use self::path::Translator as PathTranslator;
use self::place::Translator as PlaceTranslator;
use self::r#match::Analyzer as MatchAnalyzer;
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
        expression: ExpressionTree,
        translation_hint: TranslationHint,
    ) -> Result<(Element, GeneratorExpression), Error> {
        let mut result: Vec<(ExpressionTreeNode, Location)> = Vec::new();
        let mut stack: Vec<&ExpressionTree> = Vec::new();
        stack.push(&expression);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            result.push((*node.value.to_owned(), node.location));
            match node.left {
                None => {}
                Some(ref node) => stack.push(node),
            }
            match node.right {
                None => {}
                Some(ref node) => stack.push(node),
            }
        }

        for (node, location) in result.into_iter().rev() {
            match node {
                ExpressionTreeNode::Operand(operand) => self
                    .evaluation_stack
                    .push(StackElement::NotEvaluated(operand)),
                ExpressionTreeNode::Operator(ExpressionOperator::Assignment) => self.assignment(
                    GeneratorExpressionOperator::Assignment,
                    Element::assign,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentBitwiseOr) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentBitwiseOr,
                        Element::assign_bitwise_or,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentBitwiseXor) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentBitwiseXor,
                        Element::assign_bitwise_xor,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentBitwiseAnd) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentBitwiseAnd,
                        Element::assign_bitwise_and,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentBitwiseShiftLeft) => {
                    self.assignment(
                        GeneratorExpressionOperator::AssignmentBitwiseShiftLeft,
                        Element::assign_bitwise_shift_left,
                        location,
                    )?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentBitwiseShiftRight) => {
                    self.assignment(
                        GeneratorExpressionOperator::AssignmentBitwiseShiftRight,
                        Element::assign_bitwise_shift_right,
                        location,
                    )?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentAddition) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentAddition,
                        Element::assign_add,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentSubtraction) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentSubtraction,
                        Element::assign_subtract,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentMultiplication) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentMultiplication,
                        Element::assign_multiply,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentDivision) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentDivision,
                        Element::assign_divide,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::AssignmentRemainder) => self
                    .assignment(
                        GeneratorExpressionOperator::AssignmentRemainder,
                        Element::assign_remainder,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::RangeInclusive) => {
                    self.range(Element::range_inclusive, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Range) => {
                    self.range(Element::range, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Or) => {
                    self.binary(GeneratorExpressionOperator::Or, Element::or, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Xor) => {
                    self.binary(GeneratorExpressionOperator::Xor, Element::xor, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::And) => {
                    self.binary(GeneratorExpressionOperator::And, Element::and, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Equals) => self.binary(
                    GeneratorExpressionOperator::Equals,
                    Element::equals,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::NotEquals) => self.binary(
                    GeneratorExpressionOperator::NotEquals,
                    Element::not_equals,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::GreaterEquals) => self.binary(
                    GeneratorExpressionOperator::GreaterEquals,
                    Element::greater_equals,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::LesserEquals) => self.binary(
                    GeneratorExpressionOperator::LesserEquals,
                    Element::lesser_equals,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Greater) => self.binary(
                    GeneratorExpressionOperator::Greater,
                    Element::greater,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Lesser) => self.binary(
                    GeneratorExpressionOperator::Lesser,
                    Element::lesser,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseOr) => self.binary(
                    GeneratorExpressionOperator::BitwiseOr,
                    Element::bitwise_or,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseXor) => self.binary(
                    GeneratorExpressionOperator::BitwiseXor,
                    Element::bitwise_xor,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseAnd) => self.binary(
                    GeneratorExpressionOperator::BitwiseAnd,
                    Element::bitwise_and,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseShiftLeft) => self.binary(
                    GeneratorExpressionOperator::BitwiseShiftLeft,
                    Element::bitwise_shift_left,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseShiftRight) => self
                    .binary(
                        GeneratorExpressionOperator::BitwiseShiftRight,
                        Element::bitwise_shift_right,
                        location,
                    )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Addition) => self.binary(
                    GeneratorExpressionOperator::Addition,
                    Element::add,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Subtraction) => self.binary(
                    GeneratorExpressionOperator::Subtraction,
                    Element::subtract,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Multiplication) => self.binary(
                    GeneratorExpressionOperator::Multiplication,
                    Element::multiply,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Division) => self.binary(
                    GeneratorExpressionOperator::Division,
                    Element::divide,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Remainder) => self.binary(
                    GeneratorExpressionOperator::Remainder,
                    Element::remainder,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Casting) => {
                    self.casting(location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Not) => {
                    self.unary(GeneratorExpressionOperator::Not, Element::not, location)?
                }
                ExpressionTreeNode::Operator(ExpressionOperator::BitwiseNot) => self.unary(
                    GeneratorExpressionOperator::BitwiseNot,
                    Element::bitwise_not,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Negation) => self.unary(
                    GeneratorExpressionOperator::Negation,
                    Element::negate,
                    location,
                )?,
                ExpressionTreeNode::Operator(ExpressionOperator::Index) => self.index(location)?,
                ExpressionTreeNode::Operator(ExpressionOperator::Field) => self.field(location)?,
                ExpressionTreeNode::Operator(ExpressionOperator::CallBuiltIn) => {
                    self.is_next_call_builtin = true;
                }
                ExpressionTreeNode::Operator(ExpressionOperator::Call) => self.call(location)?,
                ExpressionTreeNode::Operator(ExpressionOperator::Path) => self.path(location)?,
            }
        }

        let (element, intermediate) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            translation_hint,
        )?;
        if let Some(operand) = intermediate {
            self.intermediate.push_operand(operand);
        }
        Ok((element, self.intermediate))
    }

    fn assignment<F>(
        &mut self,
        operator: GeneratorExpressionOperator,
        callback: F,
        location: Location,
    ) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<Place, ElementError>,
    {
        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PlaceExpression,
        )?;

        let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
        let place =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;

        if !place.is_mutable {
            let item_location =
                Scope::resolve_item(self.scope_stack.top(), place.identifier.as_str())
                    .map_err(|error| Error::Scope(place.location, error))?
                    .location;
            return Err(Error::MutatingImmutableMemory {
                location,
                name: place.to_string(),
                reference: item_location,
            });
        }
        if place.r#type != r#type {
            return Err(Error::MutatingWithDifferentType {
                location,
                expected: r#type.to_string(),
                found: place.r#type.to_string(),
            });
        }

        self.evaluation_stack
            .push(StackElement::Evaluated(Element::Value(Value::Unit)));

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate.push_operator(operator);

        Ok(())
    }

    fn range<F>(&mut self, callback: F, location: Location) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<Element, ElementError>,
    {
        let (operand_2, _intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, _intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let result =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    fn binary<F>(
        &mut self,
        operator: GeneratorExpressionOperator,
        callback: F,
        location: Location,
    ) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<Element, ElementError>,
    {
        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let result =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate.push_operator(operator);

        Ok(())
    }

    fn unary<F>(
        &mut self,
        operator: GeneratorExpressionOperator,
        callback: F,
        location: Location,
    ) -> Result<(), Error>
    where
        F: FnOnce(Element) -> Result<Element, ElementError>,
    {
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let result = callback(operand_1).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate.push_operator(operator);

        Ok(())
    }

    fn casting(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::TypeExpression,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let operator = match operand_2 {
            Element::Type(ref r#type) => GeneratorExpressionOperator::casting(r#type),
            _ => None,
        };

        let result =
            Element::cast(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operator) = operator {
            self.intermediate.push_operator(operator);
        }

        Ok(())
    }

    fn index(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PlaceExpression,
        )?;

        let (result, data) = Element::index(operand_1, operand_2)
            .map_err(|error| Error::Element(location, error))?;

        match result {
            operand @ Element::Place(_) => {
                self.evaluation_stack.push(StackElement::Evaluated(operand));
            }
            Element::Value(_) | Element::Constant(_) => {
                let value = Value::try_from(&data.sliced_type)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(location, error))?;
                self.evaluation_stack
                    .push(StackElement::Evaluated(Element::Value(value)));
            }
            _ => {}
        }

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate
            .push_operator(GeneratorExpressionOperator::index(data));

        Ok(())
    }

    fn field(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::CompoundTypeMember,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PlaceExpression,
        )?;

        let (result, data) = Element::field(operand_1, operand_2)
            .map_err(|error| Error::Element(location, error))?;

        match result {
            operand @ Element::Place(_) => {
                self.evaluation_stack.push(StackElement::Evaluated(operand));
            }
            Element::Value(_) | Element::Constant(_) => {
                let value = Value::try_from(&data.sliced_type)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(location, error))?;
                self.evaluation_stack
                    .push(StackElement::Evaluated(Element::Value(value)));
            }
            _ => {}
        }

        if let Some(operand) = intermediate_1 {
            self.intermediate.push_operand(operand);
        }
        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate
            .push_operator(GeneratorExpressionOperator::slice(data));

        Ok(())
    }

    fn call(&mut self, location: Location) -> Result<(), Error> {
        let is_call_builtin = self.is_next_call_builtin;
        self.is_next_call_builtin = false;

        let (operand_2, intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, _intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::TypeExpression,
        )?;

        let (element, operator) = CallAnalyzer::analyze(
            self.scope_stack.top(),
            operand_1,
            operand_2,
            is_call_builtin,
            location,
        )?;

        self.evaluation_stack.push(StackElement::Evaluated(element));

        if let Some(operand) = intermediate_2 {
            self.intermediate.push_operand(operand);
        }
        self.intermediate.push_operator(operator);

        Ok(())
    }

    fn path(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, _intermediate_2) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::CompoundTypeMember,
        )?;
        let (operand_1, _intermediate_1) = Self::evaluate_operand(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PathExpression,
        )?;

        let result =
            Element::path(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    fn evaluate_operand(
        scope: Rc<RefCell<Scope>>,
        element: StackElement,
        translation_hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match element {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok((Element::Constant(Constant::Unit), None)),
                ExpressionOperand::LiteralBoolean(literal) => LiteralAnalyzer::boolean(literal),
                ExpressionOperand::LiteralInteger(literal) => LiteralAnalyzer::integer(literal),
                ExpressionOperand::LiteralString(literal) => {
                    Ok((LiteralAnalyzer::string(literal)?, None))
                }
                ExpressionOperand::MemberInteger(integer) => {
                    Ok((MemberAnalyzer::integer(integer)?, None))
                }
                ExpressionOperand::MemberString(identifier) => {
                    Ok((MemberAnalyzer::string(identifier)?, None))
                }
                ExpressionOperand::Identifier(identifier) => {
                    IdentifierAnalyzer::analyze(scope, identifier, translation_hint)
                }
                ExpressionOperand::Type(r#type) => {
                    Ok((TypeAnalyzer::analyze(scope, r#type)?, None))
                }
                ExpressionOperand::Array(expression) => ArrayAnalyzer::analyze(scope, expression)
                    .map(|(element, intermediate)| (element, Some(intermediate))),
                ExpressionOperand::Tuple(expression) => TupleAnalyzer::analyze(scope, expression)
                    .map(|(element, intermediate)| (element, Some(intermediate))),
                ExpressionOperand::Structure(expression) => {
                    StructureAnalyzer::analyze(scope, expression)
                        .map(|(element, intermediate)| (element, Some(intermediate)))
                }
                ExpressionOperand::List(expressions) => ListAnalyzer::analyze(scope, expressions)
                    .map(|(element, intermediate)| (element, Some(intermediate))),
                ExpressionOperand::Block(expression) => BlockAnalyzer::analyze(scope, expression)
                    .map(|(element, intermediate)| {
                        (
                            element,
                            Some(GeneratorExpressionOperand::Block(intermediate)),
                        )
                    }),
                ExpressionOperand::Conditional(expression) => {
                    ConditionalAnalyzer::analyze(scope, expression)
                        .map(|(element, intermediate)| (element, Some(intermediate)))
                }
                ExpressionOperand::Match(expression) => MatchAnalyzer::analyze(scope, expression)
                    .map(|(element, intermediate)| (element, Some(intermediate))),
                ExpressionOperand::Parenthesized(expression) => {
                    let (element, intermediate) =
                        Self::new(scope).analyze(*expression, translation_hint)?;
                    Ok((
                        element,
                        Some(GeneratorExpressionOperand::Parenthesized(Box::new(
                            intermediate,
                        ))),
                    ))
                }
            },
            StackElement::Evaluated(element) => match element {
                Element::Path(path) => PathTranslator::translate(scope, &path, translation_hint),
                Element::Place(place) => {
                    Ok((PlaceTranslator::translate(&place, translation_hint)?, None))
                }
                element => Ok((element, None)),
            },
        }
    }
}
