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
use std::rc::Rc;

use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::PathElement as PlacePathElement;
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
        tree: ExpressionTree,
        hint: TranslationHint,
    ) -> Result<(Element, GeneratorExpression), Error> {
        let (element, intermediate) = self.traverse(tree, hint)?;
        if let Some(intermediate) = intermediate {
            self.intermediate.push_operand(intermediate)
        }
        if let (Element::Place(place), TranslationHint::ValueExpression) = (&element, hint) {
            self.intermediate
                .push_operand(GeneratorExpressionOperand::Place(place.to_owned()))
        }
        Ok((element, self.intermediate))
    }

    pub fn traverse(
        &mut self,
        tree: ExpressionTree,
        hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match *tree.value {
            ExpressionTreeNode::Operand(operand) => {
                return Self::evaluate(
                    self.scope_stack.top(),
                    StackElement::NotEvaluated(operand),
                    hint,
                );
            }
            ExpressionTreeNode::Operator(operator) => match operator {
                ExpressionOperator::Range => {
                    let _intermediate = self.left_global(tree.left, operator)?;
                    let _intermediate = self.right_global(tree.right, operator)?;
                    self.binary(Element::range, tree.location)?;
                    return match self.evaluation_stack.pop() {
                        StackElement::Evaluated(element) => Ok((element, None)),
                        _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                }
                ExpressionOperator::RangeInclusive => {
                    let _intermediate = self.left_global(tree.left, operator)?;
                    let _intermediate = self.right_global(tree.right, operator)?;
                    self.binary(Element::range_inclusive, tree.location)?;
                    return match self.evaluation_stack.pop() {
                        StackElement::Evaluated(element) => Ok((element, None)),
                        _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                }

                ExpressionOperator::Or => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::or, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Or);
                }
                ExpressionOperator::Xor => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::xor, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Xor);
                }
                ExpressionOperator::And => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::and, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::And);
                }

                ExpressionOperator::Equals => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::equals, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Equals);
                }
                ExpressionOperator::NotEquals => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::not_equals, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::NotEquals);
                }
                ExpressionOperator::GreaterEquals => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::greater_equals, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::GreaterEquals);
                }
                ExpressionOperator::LesserEquals => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::lesser_equals, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::LesserEquals);
                }
                ExpressionOperator::Greater => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::greater, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Greater);
                }
                ExpressionOperator::Lesser => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::lesser, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Lesser);
                }

                ExpressionOperator::BitwiseOr => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::bitwise_or, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseOr);
                }
                ExpressionOperator::BitwiseXor => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::bitwise_xor, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseXor);
                }
                ExpressionOperator::BitwiseAnd => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::bitwise_and, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseAnd);
                }
                ExpressionOperator::BitwiseShiftLeft => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::bitwise_shift_left, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseShiftLeft);
                }
                ExpressionOperator::BitwiseShiftRight => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::bitwise_shift_right, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseShiftRight);
                }

                ExpressionOperator::Addition => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::add, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Addition);
                }
                ExpressionOperator::Subtraction => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::subtract, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Subtraction);
                }
                ExpressionOperator::Multiplication => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::multiply, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Multiplication);
                }
                ExpressionOperator::Division => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::divide, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Division);
                }
                ExpressionOperator::Remainder => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.binary(Element::remainder, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Remainder);
                }

                ExpressionOperator::Casting => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    if let Some(operator) = self.casting(tree.location)? {
                        self.intermediate.push_operator(operator);
                    }
                }

                ExpressionOperator::Not => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.unary(Element::not, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Not);
                }

                ExpressionOperator::BitwiseNot => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.unary(Element::bitwise_not, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::BitwiseNot);
                }

                ExpressionOperator::Negation => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.unary(Element::negate, tree.location)?;
                    self.intermediate
                        .push_operator(GeneratorExpressionOperator::Negation);
                }

                ExpressionOperator::Index => {
                    let _intermediate = self.left_global(tree.left, operator)?;
                    let intermediate = self.right_global(tree.right, operator)?;
                    self.index(tree.location, intermediate)?;
                    // return match self.evaluation_stack.pop() {
                    //     StackElement::Evaluated(element) => Ok((element, None)),
                    //     _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
                    // };
                }

                ExpressionOperator::Field => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.field(tree.location)?;
                }

                ExpressionOperator::Path => {
                    self.left_local(tree.left, operator)?;
                    self.right_local(tree.right, operator)?;
                    self.path(tree.location)?;
                }

                _ => todo!(),
            },
        }

        Self::evaluate(self.scope_stack.top(), self.evaluation_stack.pop(), hint)
    }

    pub fn left_local(
        &mut self,
        left: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
    ) -> Result<(), Error> {
        match left {
            Some(left) => {
                let hint = TranslationHint::first(operator);
                let (element, intermediate) = self.traverse(*left, hint)?;

                self.evaluation_stack.push(StackElement::Evaluated(element));
                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }
            }
            None => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        }
        Ok(())
    }

    pub fn right_local(
        &mut self,
        right: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
    ) -> Result<(), Error> {
        match right {
            Some(right) => {
                let hint = TranslationHint::second(operator);
                let (element, intermediate) = self.traverse(*right, hint)?;

                self.evaluation_stack.push(StackElement::Evaluated(element));
                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }
            }
            None => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        }
        Ok(())
    }

    pub fn left_global(
        &mut self,
        left: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
    ) -> Result<GeneratorExpression, Error> {
        let hint = TranslationHint::first(operator);
        let (element, intermediate) = match left {
            Some(left) => Self::new(self.scope_stack.top()).analyze(*left, hint)?,
            None => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    pub fn right_global(
        &mut self,
        right: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
    ) -> Result<GeneratorExpression, Error> {
        let hint = TranslationHint::second(operator);
        let (element, intermediate) = match right {
            Some(left) => Self::new(self.scope_stack.top()).analyze(*left, hint)?,
            None => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    fn binary<F>(&mut self, callback: F, location: Location) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<Element, ElementError>,
    {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let result =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    fn casting(
        &mut self,
        location: Location,
    ) -> Result<Option<GeneratorExpressionOperator>, Error> {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::TypeExpression,
        )?;
        let (operand_1, _) = Self::evaluate(
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

        Ok(operator)
    }

    fn unary<F>(&mut self, callback: F, location: Location) -> Result<(), Error>
    where
        F: FnOnce(Element) -> Result<Element, ElementError>,
    {
        let (operand, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;

        let result = callback(operand).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    fn index(&mut self, location: Location, expression: GeneratorExpression) -> Result<(), Error> {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PlaceExpression,
        )?;

        let (result, data) = Element::index(operand_1, operand_2.clone())
            .map_err(|error| Error::Element(location, error))?;

        match result {
            Element::Place(mut place) => {
                match operand_2 {
                    Element::Constant(Constant::Range(range)) => {
                        place.push_path_element(PlacePathElement::IndexRange {
                            start: range.start,
                            end: range.end,
                            access: data,
                        });
                    }
                    Element::Constant(Constant::RangeInclusive(range)) => {
                        place.push_path_element(PlacePathElement::IndexRangeInclusive {
                            start: range.start,
                            end: range.end,
                            access: data,
                        });
                    }
                    _ => place.push_path_element(PlacePathElement::IndexExpression {
                        expression,
                        access: data,
                    }),
                }
                self.evaluation_stack
                    .push(StackElement::Evaluated(Element::Place(place)));
            }
            element => self.evaluation_stack.push(StackElement::Evaluated(element)),
        }

        Ok(())
    }

    fn field(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::CompoundTypeMember,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PlaceExpression,
        )?;

        let (result, data) = Element::field(operand_1, operand_2)
            .map_err(|error| Error::Element(location, error))?;

        match result {
            Element::Place(mut place) => {
                place.push_path_element(PlacePathElement::Field { access: data });
                self.evaluation_stack
                    .push(StackElement::Evaluated(Element::Place(place)));
            }
            element => self.evaluation_stack.push(StackElement::Evaluated(element)),
        }

        Ok(())
    }

    fn path(&mut self, location: Location) -> Result<(), Error> {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::CompoundTypeMember,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::PathExpression,
        )?;

        let result =
            Element::path(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
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
        let (operand_2, intermediate_2) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, intermediate_1) = Self::evaluate(
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

    fn call(&mut self, location: Location) -> Result<(), Error> {
        let is_call_builtin = self.is_next_call_builtin;
        self.is_next_call_builtin = false;

        let (operand_2, intermediate_2) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationHint::ValueExpression,
        )?;
        let (operand_1, _intermediate_1) = Self::evaluate(
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

    fn evaluate(
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
                Element::Place(place) => PlaceTranslator::translate(place, translation_hint),
                element => Ok((element, None)),
            },
        }
    }
}
