//!
//! The expression semantic analyzer.
//!

mod tests;

pub mod array;
pub mod block;
pub mod call;
pub mod conditional;
pub mod error;
pub mod field_index;
pub mod identifier;
pub mod list;
pub mod literal;
pub mod r#match;
pub mod path;
pub mod place;
pub mod stack;
pub mod structure;
pub mod tuple;
pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::constant::integer::Integer as GeneratorExpressionIntegerConstant;
use crate::generator::expression::operand::constant::Constant as GeneratorExpressionConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::access::FieldVariant as FieldAccessVariant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::element::Element as PlaceElement;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

use self::array::Analyzer as ArrayAnalyzer;
use self::block::Analyzer as BlockAnalyzer;
use self::call::r#type::Type as CallType;
use self::call::Analyzer as CallAnalyzer;
use self::conditional::Analyzer as ConditionalAnalyzer;
use self::field_index::Analyzer as TupleIndexAnalyzer;
use self::identifier::Analyzer as IdentifierAnalyzer;
use self::list::Analyzer as ListAnalyzer;
use self::literal::Analyzer as LiteralAnalyzer;
use self::path::Translator as PathTranslator;
use self::place::Translator as PlaceTranslator;
use self::r#match::Analyzer as MatchAnalyzer;
use self::r#type::Analyzer as TypeAnalyzer;
use self::stack::element::Element as StackElement;
use self::stack::Stack as EvaluationStack;
use self::structure::Analyzer as StructureAnalyzer;
use self::tuple::Analyzer as TupleAnalyzer;

///
/// The expression semantic analyzer.
///
/// Produces the IR tree.
///
pub struct Analyzer {
    scope_stack: ScopeStack,
    evaluation_stack: EvaluationStack,
    intermediate: GeneratorExpression,
    rule: TranslationRule,
    next_call_type: CallType,
}

impl Analyzer {
    ///
    /// Initializes a new analyzer with access to the `scope`.
    ///
    pub fn new(scope: Rc<RefCell<Scope>>, rule: TranslationRule) -> Self {
        Self {
            scope_stack: ScopeStack::new(scope),
            evaluation_stack: EvaluationStack::new(),
            intermediate: GeneratorExpression::new(),
            rule,
            next_call_type: CallType::Normal,
        }
    }

    ///
    /// Analyzes the expression `tree`, producing a semantic element and the IR tree.
    ///
    /// Returns the whole IR subtree, which is useful to postpone writing the result
    /// to the main IR tree.
    ///
    /// Is used either as an outer interface method, or for getting the array index 2nd operand
    /// subtree, assignment operands and so on.
    ///
    pub fn analyze(
        mut self,
        tree: ExpressionTree,
    ) -> Result<(Element, GeneratorExpression), Error> {
        log::trace!("Analyzing an expression tree");

        let (element, intermediate) = self.traverse(tree, self.rule)?;
        if let Some(intermediate) = intermediate {
            self.intermediate.push_operand(intermediate)
        }

        if let (Element::Place(place), TranslationRule::Value) = (&element, self.rule) {
            self.intermediate
                .push_operand(GeneratorExpressionOperand::Place(place.to_owned().into()))
        }

        Ok((element, self.intermediate))
    }

    ///
    /// Analyzes the expression `tree`, producing a semantic element and the IR expression result.
    ///
    /// Returns only the result for the IR, writing all the intermediate results to
    /// `self.intermediate` on the fly.
    ///
    /// Is used for ordinar expressions with sequences of logical, bitwise, arithmetic operators,
    /// where it is simpler to write operands at once, since there are no options to write them
    /// differently like in cases where place or path expressions are involved.
    ///
    pub fn traverse(
        &mut self,
        tree: ExpressionTree,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        log::trace!("Traversing an expression tree");

        match *tree.value {
            ExpressionTreeNode::Operand(operand) => {
                log::trace!("Traversing an expression tree operand");

                return Self::evaluate(
                    self.scope_stack.top(),
                    StackElement::NotEvaluated(operand),
                    rule,
                );
            }
            ExpressionTreeNode::Operator(operator) => {
                log::trace!("Traversing an expression tree operator");

                match operator {
                    ExpressionOperator::Assignment => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, _operator) = self.assignment(Element::assign, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::Assignment {
                                place: place.into(),
                                expression,
                            },
                        );
                    }
                    ExpressionOperator::AssignmentBitwiseOr => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_bitwise_or, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentBitwiseOr {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentBitwiseXor => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_bitwise_xor, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentBitwiseXor {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentBitwiseAnd => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_bitwise_and, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentBitwiseAnd {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentBitwiseShiftLeft => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_bitwise_shift_left, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentBitwiseShiftLeft {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentBitwiseShiftRight => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_bitwise_shift_right, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentBitwiseShiftRight {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentAddition => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_add, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentAddition {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentSubtraction => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_subtract, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentSubtraction {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentMultiplication => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_multiply, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentMultiplication {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentDivision => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_divide, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentDivision {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }
                    ExpressionOperator::AssignmentRemainder => {
                        self.left_global(tree.left, operator, rule)?;
                        let expression = self.right_global(tree.right, operator, rule)?;
                        let (place, operator) =
                            self.assignment(Element::assign_remainder, tree.location)?;
                        self.intermediate.push_operator(
                            tree.location,
                            GeneratorExpressionOperator::AssignmentRemainder {
                                place: place.into(),
                                expression,
                                operator: Box::new(operator),
                            },
                        );
                    }

                    ExpressionOperator::Range => {
                        let _intermediate = self.left_global(tree.left, operator, rule)?;
                        let _intermediate = self.right_global(tree.right, operator, rule)?;

                        let intermediate = self.range(Element::range, tree.location)?;

                        return match self.evaluation_stack.pop() {
                            StackElement::Evaluated(element) => Ok((element, Some(intermediate))),
                            _ => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
                        };
                    }
                    ExpressionOperator::RangeInclusive => {
                        let _intermediate = self.left_global(tree.left, operator, rule)?;
                        let _intermediate = self.right_global(tree.right, operator, rule)?;
                        let intermediate = self.range(Element::range_inclusive, tree.location)?;

                        return match self.evaluation_stack.pop() {
                            StackElement::Evaluated(element) => Ok((element, Some(intermediate))),
                            _ => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
                        };
                    }

                    ExpressionOperator::Or => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::or, tree.location)?;
                    }
                    ExpressionOperator::Xor => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::xor, tree.location)?;
                    }
                    ExpressionOperator::And => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::and, tree.location)?;
                    }

                    ExpressionOperator::Equals => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::equals, tree.location)?;
                    }
                    ExpressionOperator::NotEquals => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::not_equals, tree.location)?;
                    }
                    ExpressionOperator::GreaterEquals => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::greater_equals, tree.location)?;
                    }
                    ExpressionOperator::LesserEquals => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::lesser_equals, tree.location)?;
                    }
                    ExpressionOperator::Greater => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::greater, tree.location)?;
                    }
                    ExpressionOperator::Lesser => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::lesser, tree.location)?;
                    }

                    ExpressionOperator::BitwiseOr => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::bitwise_or, tree.location)?;
                    }
                    ExpressionOperator::BitwiseXor => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::bitwise_xor, tree.location)?;
                    }
                    ExpressionOperator::BitwiseAnd => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::bitwise_and, tree.location)?;
                    }
                    ExpressionOperator::BitwiseShiftLeft => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::bitwise_shift_left, tree.location)?;
                    }
                    ExpressionOperator::BitwiseShiftRight => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::bitwise_shift_right, tree.location)?;
                    }

                    ExpressionOperator::Addition => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::add, tree.location)?;
                    }
                    ExpressionOperator::Subtraction => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::subtract, tree.location)?;
                    }
                    ExpressionOperator::Multiplication => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::multiply, tree.location)?;
                    }
                    ExpressionOperator::Division => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::divide, tree.location)?;
                    }
                    ExpressionOperator::Remainder => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.binary(Element::remainder, tree.location)?;
                    }

                    ExpressionOperator::Casting => {
                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.casting(tree.location)?;
                    }

                    ExpressionOperator::Not => {
                        self.left_local(tree.left, operator, rule)?;
                        self.unary(Element::not, tree.location)?;
                    }

                    ExpressionOperator::BitwiseNot => {
                        self.left_local(tree.left, operator, rule)?;
                        self.unary(Element::bitwise_not, tree.location)?;
                    }

                    ExpressionOperator::Negation => {
                        self.left_local(tree.left, operator, rule)?;
                        self.unary(Element::negate, tree.location)?;
                    }

                    ExpressionOperator::Index => {
                        log::trace!("Traversing an expression tree operator index");

                        self.left_local(tree.left, operator, rule)?;
                        let intermediate = self.right_global(tree.right, operator, rule)?;
                        let intermediate = self.index(tree.location, intermediate)?;
                        if let Some(intermediate) = intermediate {
                            self.intermediate.push_operator(tree.location, intermediate);
                        }
                    }

                    ExpressionOperator::Field => {
                        log::trace!("Traversing an expression tree operator field");

                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        let intermediate = self.field(tree.location)?;
                        if let Some(intermediate) = intermediate {
                            self.intermediate.push_operator(tree.location, intermediate);
                        }
                    }

                    ExpressionOperator::Call => {
                        log::trace!("Traversing an expression tree operator call");

                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        let intermediate = self.call(tree.location, rule)?;
                        self.intermediate.push_operator(tree.location, intermediate);
                    }
                    ExpressionOperator::CallBuiltIn => {
                        log::trace!("Traversing an expression tree operator call builtin");

                        self.next_call_type = CallType::BuiltIn;
                        self.left_local(tree.left, operator, rule)?;
                    }

                    ExpressionOperator::Path => {
                        log::trace!("Traversing an expression tree operator path");

                        self.left_local(tree.left, operator, rule)?;
                        self.right_local(tree.right, operator, rule)?;
                        self.path(tree.location)?;
                    }
                }
            }
        }

        Self::evaluate(self.scope_stack.top(), self.evaluation_stack.pop(), rule)
    }

    ///
    /// Analyzes the left operand of a binary operand.
    ///
    /// Writes results to the current evaluation stack and IR instance.
    ///
    pub fn left_local(
        &mut self,
        left: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<(), Error> {
        match left {
            Some(left) => {
                log::trace!("Analyzing a left operand locally: {:?}", left);

                let rule = TranslationRule::first(operator, rule);
                let (element, intermediate) = self.traverse(*left, rule)?;

                self.evaluation_stack.push(StackElement::Evaluated(element));
                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }

                Ok(())
            }
            None => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        }
    }

    ///
    /// Analyzes the right operand of a binary operand.
    ///
    /// Writes results to the current evaluation stack and IR instance.
    ///
    pub fn right_local(
        &mut self,
        right: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<(), Error> {
        match right {
            Some(right) => {
                log::trace!("Analyzing a right operand locally: {:?}", right);

                let rule = TranslationRule::second(operator, rule);
                let (element, intermediate) = self.traverse(*right, rule)?;

                self.evaluation_stack.push(StackElement::Evaluated(element));
                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }

                Ok(())
            }
            None => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        }
    }

    ///
    /// Analyzes the left operand of a binary operand.
    ///
    /// Creates a new analyzer to avoid writing to the current evaluation stack and IR instance.
    ///
    pub fn left_global(
        &mut self,
        left: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<GeneratorExpression, Error> {
        log::trace!("Analyzing a left operand separately");

        let rule = TranslationRule::first(operator, rule);
        let (element, intermediate) = match left {
            Some(left) => Self::new(self.scope_stack.top(), rule).analyze(*left)?,
            None => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    ///
    /// Analyzes the right operand of a binary operand.
    ///
    /// Creates a new analyzer to avoid writing to the current evaluation stack and IR instance.
    ///
    pub fn right_global(
        &mut self,
        right: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<GeneratorExpression, Error> {
        log::trace!("Analyzing a right operand separately");

        let rule = TranslationRule::second(operator, rule);
        let (element, intermediate) = match right {
            Some(left) => Self::new(self.scope_stack.top(), rule).analyze(*left)?,
            None => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    ///
    /// Analyzes the assignment operation.
    ///
    fn assignment<F>(
        &mut self,
        callback: F,
        location: Location,
    ) -> Result<(Place, GeneratorExpressionOperator), Error>
    where
        F: FnOnce(Element, Element) -> Result<(Place, GeneratorExpressionOperator), ElementError>,
    {
        log::trace!("Analyzing an assignment operation");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Value,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Place,
        )?;

        let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
        let (place, operator) =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;

        if !place.is_mutable {
            let item_location =
                Scope::resolve_item(self.scope_stack.top(), &place.identifier, true)?.location;
            return Err(Error::Element(
                location,
                ElementError::Place(PlaceError::MutatingImmutableMemory {
                    name: place.to_string(),
                    reference: item_location,
                }),
            ));
        }
        if place.r#type != r#type {
            return Err(Error::Element(
                location,
                ElementError::Place(PlaceError::MutatingWithDifferentType {
                    expected: r#type.to_string(),
                    found: place.r#type.to_string(),
                }),
            ));
        }

        self.evaluation_stack
            .push(StackElement::Evaluated(Element::Value(Value::Unit)));

        Ok((place, operator))
    }

    ///
    /// Analyzes the binary operation, which can be logical, comparison, bitwise or arithmetic.
    ///
    fn binary<F>(&mut self, callback: F, location: Location) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<(Element, GeneratorExpressionOperator), ElementError>,
    {
        log::trace!("Analyzing a binary operation");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;

        let (result, operator) =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));
        self.intermediate.push_operator(location, operator);

        Ok(())
    }

    ///
    /// Analyzes the range operation, returns the range start value as the IR expression operand.
    ///
    fn range<F>(
        &mut self,
        callback: F,
        location: Location,
    ) -> Result<GeneratorExpressionOperand, Error>
    where
        F: FnOnce(Element, Element) -> Result<Element, ElementError>,
    {
        log::trace!("Analyzing range");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;

        let result =
            callback(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        let start = match result {
            Element::Constant(Constant::Range(ref range)) => range.start.to_owned(),
            Element::Constant(Constant::RangeInclusive(ref range)) => range.start.to_owned(),
            _ => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        let intermediate =
            GeneratorExpressionOperand::Constant(GeneratorExpressionConstant::Integer(
                GeneratorExpressionIntegerConstant::new(start, false, crate::BITLENGTH_FIELD),
            ));

        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(intermediate)
    }

    ///
    /// Analyzes the casting operation.
    ///
    fn casting(&mut self, location: Location) -> Result<(), Error> {
        log::trace!("Analyzing casting");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Type,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;

        let (result, operator) =
            Element::cast(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));
        if let Some(operator) = operator {
            self.intermediate.push_operator(location, operator);
        }

        Ok(())
    }

    ///
    /// Analyzes the unary operation.
    ///
    fn unary<F>(&mut self, callback: F, location: Location) -> Result<(), Error>
    where
        F: FnOnce(Element) -> Result<(Element, GeneratorExpressionOperator), ElementError>,
    {
        log::trace!("Analyzing an unary operation");

        let (operand, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;

        let (result, operator) =
            callback(operand).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));
        self.intermediate.push_operator(location, operator);

        Ok(())
    }

    ///
    /// Analyzes the array index operation.
    ///
    fn index(
        &mut self,
        location: Location,
        expression: GeneratorExpression,
    ) -> Result<Option<GeneratorExpressionOperator>, Error> {
        log::trace!("Analyzing index");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Place,
        )?;

        let (result, access) = Element::index(operand_1, operand_2.clone())
            .map_err(|error| Error::Element(location, error))?;

        match result {
            Element::Place(mut place) => {
                match operand_2 {
                    Element::Constant(Constant::Range(range)) => {
                        place.push_element(PlaceElement::IndexRange {
                            start: range.start,
                            end: range.end,
                            access,
                        });
                    }
                    Element::Constant(Constant::RangeInclusive(range)) => {
                        place.push_element(PlaceElement::IndexRangeInclusive {
                            start: range.start,
                            end: range.end,
                            access,
                        });
                    }
                    Element::Constant(Constant::Integer(integer)) => {
                        place.push_element(PlaceElement::IndexConstant {
                            constant: integer,
                            access,
                        })
                    }
                    _ => place.push_element(PlaceElement::IndexExpression { expression, access }),
                }

                self.evaluation_stack
                    .push(StackElement::Evaluated(Element::Place(place)));

                Ok(None)
            }
            element => {
                self.evaluation_stack.push(StackElement::Evaluated(element));

                Ok(Some(GeneratorExpressionOperator::index(expression, access)))
            }
        }
    }

    ///
    /// Analyzes the tuple or structure field access operation.
    ///
    fn field(&mut self, location: Location) -> Result<Option<GeneratorExpressionOperator>, Error> {
        log::trace!("Analyzing field");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Field,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Place,
        )?;

        let (result, access) = Element::field(operand_1, operand_2)
            .map_err(|error| Error::Element(location, error))?;

        match access {
            FieldAccessVariant::Field(access) => match result {
                Element::Place(mut place) => {
                    place.push_element(PlaceElement::Field { access });

                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Place(place)));

                    Ok(None)
                }
                element => {
                    self.evaluation_stack.push(StackElement::Evaluated(element));

                    Ok(Some(GeneratorExpressionOperator::slice(access)))
                }
            },
            FieldAccessVariant::Method(instance) => {
                let instance = if let Element::Place(instance) = instance {
                    let (instance, intermedidate) = Self::evaluate(
                        self.scope_stack.top(),
                        StackElement::Evaluated(Element::Place(instance)),
                        TranslationRule::Value,
                    )?;

                    if let Some(instance) = intermedidate {
                        self.intermediate.push_operand(instance);
                    }

                    instance
                } else {
                    instance
                };

                self.evaluation_stack.push(StackElement::Evaluated(result));
                self.next_call_type = CallType::Method { instance };

                Ok(None)
            }
        }
    }

    ///
    /// Analyzes the function call operation.
    ///
    fn call(
        &mut self,
        location: Location,
        rule: TranslationRule,
    ) -> Result<GeneratorExpressionOperator, Error> {
        log::trace!("Analyzing call");

        let call_type = self.next_call_type.take();

        let (operand_2, _intermediate_2) =
            Self::evaluate(self.scope_stack.top(), self.evaluation_stack.pop(), rule)?;
        let (operand_1, _intermediate_1) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Type,
        )?;

        let (element, operator) = CallAnalyzer::analyze(
            self.scope_stack.top(),
            operand_1,
            operand_2,
            call_type,
            location,
        )?;

        self.evaluation_stack.push(StackElement::Evaluated(element));

        Ok(operator)
    }

    ///
    /// Analyzes the path resolution operation.
    ///
    fn path(&mut self, location: Location) -> Result<(), Error> {
        log::trace!("Analyzing path");

        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Field,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Path,
        )?;

        let result =
            Element::path(operand_1, operand_2).map_err(|error| Error::Element(location, error))?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    ///
    /// Evaluates the element, turning it into the state specified with `rule`.
    ///
    fn evaluate(
        scope: Rc<RefCell<Scope>>,
        element: StackElement,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        log::trace!("Evaluating operand");

        match element {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::LiteralUnit => Ok((Element::Constant(Constant::Unit), None)),
                ExpressionOperand::LiteralBoolean(inner) => LiteralAnalyzer::boolean(inner),
                ExpressionOperand::LiteralInteger(inner) => LiteralAnalyzer::integer(inner),
                ExpressionOperand::LiteralString(inner) => {
                    Ok((LiteralAnalyzer::string(inner)?, None))
                }
                ExpressionOperand::TupleIndex(inner) => {
                    Ok((TupleIndexAnalyzer::integer(inner)?, None))
                }
                ExpressionOperand::Identifier(inner) => {
                    IdentifierAnalyzer::analyze(scope, inner, rule)
                }
                ExpressionOperand::Type(inner) => Ok((TypeAnalyzer::analyze(scope, inner)?, None)),
                ExpressionOperand::Array(inner) => ArrayAnalyzer::analyze(scope, inner, rule),
                ExpressionOperand::Tuple(inner) => TupleAnalyzer::analyze(scope, inner, rule),
                ExpressionOperand::Structure(inner) => {
                    StructureAnalyzer::analyze(scope, inner, rule)
                }
                ExpressionOperand::List(inner) => ListAnalyzer::analyze(scope, inner, rule)
                    .map(|(element, intermediate)| (element, Some(intermediate))),
                ExpressionOperand::Block(inner) => {
                    BlockAnalyzer::analyze(scope, inner, rule).map(|(element, intermediate)| {
                        (
                            element,
                            Some(GeneratorExpressionOperand::Block(intermediate)),
                        )
                    })
                }
                ExpressionOperand::Conditional(inner) => {
                    ConditionalAnalyzer::analyze(scope, inner, rule)
                }
                ExpressionOperand::Match(inner) => MatchAnalyzer::analyze(scope, inner, rule),
            },
            StackElement::Evaluated(inner) => match inner {
                Element::Path(path) => PathTranslator::translate(scope, path, rule),
                Element::Place(place) => PlaceTranslator::translate(place, rule),
                element => Ok((element, None)),
            },
        }
    }
}
