//!
//! The expression semantic analyzer.
//!

#[cfg(test)]
mod tests;

pub mod array;
pub mod block;
pub mod call;
pub mod conditional;
pub mod identifier;
pub mod list;
pub mod literal;
pub mod r#match;
pub mod path;
pub mod place;
pub mod stack;
pub mod structure;
pub mod tuple;
pub mod tuple_index;
pub mod r#type;

use std::cell::RefCell;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::ExpressionOperand;
use zinc_syntax::ExpressionOperator;
use zinc_syntax::ExpressionTree;
use zinc_syntax::ExpressionTreeNode;

use crate::generator::expression::element::Element as GeneratorExpressionElement;
use crate::generator::expression::operand::constant::integer::Integer as GeneratorExpressionIntegerConstant;
use crate::generator::expression::operand::constant::Constant as GeneratorExpressionConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::access::dot::Dot as DotAccess;
use crate::semantic::element::constant::unit::Unit as UnitConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::place::element::Element as PlaceElement;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::unit::Unit as UnitValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

use self::array::Analyzer as ArrayAnalyzer;
use self::block::Analyzer as BlockAnalyzer;
use self::call::r#type::Type as CallType;
use self::call::Analyzer as CallAnalyzer;
use self::conditional::Analyzer as ConditionalAnalyzer;
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
use self::tuple_index::Analyzer as TupleIndexAnalyzer;

///
/// The expression semantic analyzer.
///
/// Produces the IR tree.
///
pub struct Analyzer {
    /// The scope stack, where the expression elements are resolved.
    scope_stack: ScopeStack,
    /// The evaluation stack, where the expression is executed.
    evaluation_stack: EvaluationStack,
    /// The bytecode generator intermediate representation, generated during the expression analysis.
    intermediate: GeneratorExpression,
    /// The translation rule, which hints, which kind of element is expected as the expression result.
    rule: TranslationRule,
    /// The function call type variable, which indicates what kind of function will be called next.
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
            next_call_type: CallType::Default,
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
        match *tree.value {
            ExpressionTreeNode::Operand(operand) => {
                return Self::evaluate(
                    self.scope_stack.top(),
                    StackElement::NotEvaluated(operand),
                    rule,
                );
            }
            ExpressionTreeNode::Operator(operator) => match operator {
                ExpressionOperator::Assignment => {
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, _operator) = self.assignment(Element::assign)?;
                    self.intermediate.push_operator(
                        tree.location,
                        GeneratorExpressionOperator::Assignment {
                            place: place.into(),
                            expression,
                        },
                    );
                }
                ExpressionOperator::AssignmentBitwiseOr => {
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_bitor)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_bitxor)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_bitand)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_bitwise_shift_left)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_bitwise_shift_right)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_add)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_subtract)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_multiply)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_divide)?;
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
                    self.left_separate(tree.left, operator, rule)?;
                    let expression = self.right_separate(tree.right, operator, rule)?;
                    let (place, operator) = self.assignment(Element::assign_remainder)?;
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
                    let _intermediate = self.left_separate(tree.left, operator, rule)?;
                    let _intermediate = self.right_separate(tree.right, operator, rule)?;

                    let intermediate = self.range(Element::range)?;

                    return match self.evaluation_stack.pop() {
                        StackElement::Evaluated(element) => Ok((element, Some(intermediate))),
                        _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                }
                ExpressionOperator::RangeInclusive => {
                    let _intermediate = self.left_separate(tree.left, operator, rule)?;
                    let _intermediate = self.right_separate(tree.right, operator, rule)?;
                    let intermediate = self.range(Element::range_inclusive)?;

                    return match self.evaluation_stack.pop() {
                        StackElement::Evaluated(element) => Ok((element, Some(intermediate))),
                        _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                }

                ExpressionOperator::Or => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::or, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::Xor => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::xor, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::And => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::and, tree.location, intermediate_1, intermediate_2)?;
                }

                ExpressionOperator::Equals => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::equals,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::NotEquals => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::not_equals,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::GreaterEquals => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::greater_equals,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::LesserEquals => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::lesser_equals,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::Greater => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::greater,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::Lesser => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::lesser,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }

                ExpressionOperator::BitwiseOr => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::bitor,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::BitwiseXor => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::bitxor,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::BitwiseAnd => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(
                        Element::bitand,
                        tree.location,
                        intermediate_1,
                        intermediate_2,
                    )?;
                }
                ExpressionOperator::BitwiseShiftLeft => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::shl, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::BitwiseShiftRight => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::shr, tree.location, intermediate_1, intermediate_2)?;
                }

                ExpressionOperator::Addition => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::add, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::Subtraction => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::sub, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::Multiplication => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::mul, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::Division => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::div, tree.location, intermediate_1, intermediate_2)?;
                }
                ExpressionOperator::Remainder => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    self.binary(Element::rem, tree.location, intermediate_1, intermediate_2)?;
                }

                ExpressionOperator::Casting => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;
                    self.left_separate(tree.right, operator, rule)?;

                    self.casting(tree.location, intermediate_1)?;
                }

                ExpressionOperator::Not => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;

                    self.unary(Element::not, tree.location, intermediate_1)?;
                }

                ExpressionOperator::BitwiseNot => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;

                    self.unary(Element::bitwise_not, tree.location, intermediate_1)?;
                }

                ExpressionOperator::Negation => {
                    let intermediate_1 = self.left_separate(tree.left, operator, rule)?;

                    self.unary(Element::neg, tree.location, intermediate_1)?;
                }

                ExpressionOperator::Index => {
                    self.left_local(tree.left, operator, rule)?;
                    let intermediate_2 = self.right_separate(tree.right, operator, rule)?;

                    let intermediate = self.index(intermediate_2)?;
                    if let Some(intermediate) = intermediate {
                        self.intermediate.push_operator(tree.location, intermediate);
                    }
                }

                ExpressionOperator::Dot => {
                    let _ = self.left_local(tree.left, operator, rule)?;
                    let _ = self.right_local(tree.right, operator, rule)?;

                    let intermediate = self.dot()?;
                    if let Some(intermediate) = intermediate {
                        self.intermediate.push_operator(tree.location, intermediate);
                    }
                }

                ExpressionOperator::Call => {
                    self.left_local(tree.left, operator, rule)?;

                    // forces the constant translation rule, which prevents the arguments to be written to the IR
                    let rule = match self.evaluation_stack.top() {
                        StackElement::Evaluated(Element::Type(Type::Function(
                            FunctionType::Constant(_),
                        ))) => TranslationRule::Constant,
                        _element => self.rule,
                    };

                    self.right_local(tree.right, operator, rule)?;

                    let intermediate = self.call(tree.location, rule)?;
                    self.intermediate.push_element(intermediate);
                }
                ExpressionOperator::CallIntrinsic => {
                    self.next_call_type = CallType::MacroLike;

                    self.left_local(tree.left, operator, rule)?;
                }

                ExpressionOperator::Path => {
                    self.left_local(tree.left, operator, rule)?;
                    self.right_local(tree.right, operator, rule)?;

                    self.path()?;
                }

                ExpressionOperator::Structure => {
                    self.left_local(tree.left, operator, rule)?;
                    self.right_local(tree.right, operator, rule)?;

                    self.structure()?;
                }
            },
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
                let rule = TranslationRule::first(operator, rule);
                let (element, intermediate) = self.traverse(*left, rule)?;

                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }

                self.evaluation_stack.push(StackElement::Evaluated(element));
                Ok(())
            }
            None => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
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
                let rule = TranslationRule::second(operator, rule);
                let (element, intermediate) = self.traverse(*right, rule)?;

                if let Some(intermediate) = intermediate {
                    self.intermediate.push_operand(intermediate);
                }

                self.evaluation_stack.push(StackElement::Evaluated(element));
                Ok(())
            }
            None => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        }
    }

    ///
    /// Analyzes the left operand of a binary operand.
    ///
    /// Creates a new analyzer to avoid writing to the current evaluation stack and IR instance.
    ///
    pub fn left_separate(
        &mut self,
        left: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<GeneratorExpression, Error> {
        let rule = TranslationRule::first(operator, rule);
        let (element, intermediate) = match left {
            Some(left) => Self::new(self.scope_stack.top(), rule).analyze(*left)?,
            None => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    ///
    /// Analyzes the right operand of a binary operand.
    ///
    /// Creates a new analyzer to avoid writing to the current evaluation stack and IR instance.
    ///
    pub fn right_separate(
        &mut self,
        right: Option<Box<ExpressionTree>>,
        operator: ExpressionOperator,
        rule: TranslationRule,
    ) -> Result<GeneratorExpression, Error> {
        let rule = TranslationRule::second(operator, rule);
        let (element, intermediate) = match right {
            Some(right) => Self::new(self.scope_stack.top(), rule).analyze(*right)?,
            None => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        self.evaluation_stack.push(StackElement::Evaluated(element));
        Ok(intermediate)
    }

    ///
    /// Analyzes the assignment operation.
    ///
    fn assignment<F>(&mut self, callback: F) -> Result<(Place, GeneratorExpressionOperator), Error>
    where
        F: FnOnce(Element, Element) -> Result<(Place, GeneratorExpressionOperator), Error>,
    {
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

        let location = operand_1.location();

        let r#type = Type::from_element(&operand_2, self.scope_stack.top())?;
        let (place, operator) = callback(operand_1, operand_2)?;

        if let Some(name) = place.check_immutable_field() {
            return Err(Error::MutatingImmutableContractField {
                location: place.identifier.location,
                name,
            });
        }
        if !place.is_mutable {
            let item_location = self
                .scope_stack
                .top()
                .borrow()
                .resolve_item(&place.identifier, true)?
                .borrow()
                .location();

            return Err(Error::MutatingImmutableMemory {
                location: place.identifier.location,
                name: place.identifier.name,
                reference: item_location,
            });
        }
        if place.r#type != r#type {
            return Err(Error::MutatingWithDifferentType {
                location: place.identifier.location,
                expected: r#type.to_string(),
                found: place.r#type.to_string(),
            });
        }

        self.evaluation_stack
            .push(StackElement::Evaluated(Element::Value(Value::Unit(
                UnitValue::new(location),
            ))));

        Ok((place, operator))
    }

    ///
    /// Analyzes the binary operation, which can be logical, comparison, bitwise or arithmetic.
    ///
    fn binary<F>(
        &mut self,
        callback: F,
        location: Location,
        intermediate_1: GeneratorExpression,
        intermediate_2: GeneratorExpression,
    ) -> Result<(), Error>
    where
        F: FnOnce(Element, Element) -> Result<(Element, GeneratorExpressionOperator), Error>,
    {
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

        let (result, operator) = callback(operand_1, operand_2)?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        self.intermediate.append_expression(intermediate_1);
        if let Some(r#type) = operator.operand_1_inferred_type() {
            self.intermediate
                .push_operator(location, GeneratorExpressionOperator::casting(r#type));
        }
        match operator {
            GeneratorExpressionOperator::Or => {
                self.intermediate
                    .push_operator(location, GeneratorExpressionOperator::OrShortCircuitStart);
            }
            GeneratorExpressionOperator::And => {
                self.intermediate
                    .push_operator(location, GeneratorExpressionOperator::AndShortCircuitStart);
            }
            _ => {}
        }
        self.intermediate.append_expression(intermediate_2);
        if let Some(r#type) = operator.operand_2_inferred_type() {
            self.intermediate
                .push_operator(location, GeneratorExpressionOperator::casting(r#type));
        }
        match operator {
            GeneratorExpressionOperator::Or => {
                self.intermediate
                    .push_operator(location, GeneratorExpressionOperator::OrShortCircuitEnd);
            }
            GeneratorExpressionOperator::And => {
                self.intermediate
                    .push_operator(location, GeneratorExpressionOperator::AndShortCircuitEnd);
            }
            _ => self.intermediate.push_operator(location, operator),
        }

        Ok(())
    }

    ///
    /// Analyzes the range operation, returns the range start value as the IR expression operand.
    ///
    fn range<F>(&mut self, callback: F) -> Result<GeneratorExpressionOperand, Error>
    where
        F: FnOnce(Element, Element) -> Result<Element, Error>,
    {
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

        let result = callback(operand_1, operand_2)?;
        let start = match result {
            Element::Constant(Constant::Range(ref range)) => range.start.to_owned(),
            Element::Constant(Constant::RangeInclusive(ref range)) => range.start.to_owned(),
            _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        let intermediate =
            GeneratorExpressionOperand::Constant(GeneratorExpressionConstant::Integer(
                GeneratorExpressionIntegerConstant::new(start, false, zinc_const::bitlength::FIELD),
            ));

        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(intermediate)
    }

    ///
    /// Analyzes the casting operation.
    ///
    fn casting(
        &mut self,
        location: Location,
        intermediate_1: GeneratorExpression,
    ) -> Result<(), Error> {
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

        let (result, operator) = Element::cast(operand_1, operand_2)?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        self.intermediate.append_expression(intermediate_1);
        if let Some(operator) = operator {
            self.intermediate.push_operator(location, operator);
        }

        Ok(())
    }

    ///
    /// Analyzes the unary operation.
    ///
    fn unary<F>(
        &mut self,
        callback: F,
        location: Location,
        intermediate_1: GeneratorExpression,
    ) -> Result<(), Error>
    where
        F: FnOnce(Element) -> Result<(Element, GeneratorExpressionOperator), Error>,
    {
        let (operand, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;

        let (result, operator) = callback(operand)?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        self.intermediate.append_expression(intermediate_1);
        self.intermediate.push_operator(location, operator);

        Ok(())
    }

    ///
    /// Analyzes the array index operation.
    ///
    fn index(
        &mut self,
        expression: GeneratorExpression,
    ) -> Result<Option<GeneratorExpressionOperator>, Error> {
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

        let (result, access) = Element::index(operand_1, operand_2.clone())?;

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
    fn dot(&mut self) -> Result<Option<GeneratorExpressionOperator>, Error> {
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

        let (result, access) = Element::dot(operand_1, operand_2)?;

        match access {
            DotAccess::StackField(access) => match result {
                Element::Place(mut place) => {
                    place.push_element(PlaceElement::StackField { access });

                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Place(place)));

                    Ok(None)
                }
                element => {
                    self.evaluation_stack.push(StackElement::Evaluated(element));

                    Ok(Some(GeneratorExpressionOperator::slice(access)))
                }
            },
            DotAccess::ContractField(access) => match result {
                Element::Place(mut place) => {
                    place.push_element(PlaceElement::ContractField { access });

                    self.evaluation_stack
                        .push(StackElement::Evaluated(Element::Place(place)));

                    Ok(None)
                }
                element => {
                    self.evaluation_stack.push(StackElement::Evaluated(element));

                    Ok(None)
                }
            },
            DotAccess::Method { instance } => {
                let (instance, is_mutable) = if let Element::Place(instance) = *instance {
                    let is_mutable = instance.is_mutable;

                    let (instance, intermedidate) = Self::evaluate(
                        self.scope_stack.top(),
                        StackElement::Evaluated(Element::Place(instance)),
                        TranslationRule::Value,
                    )?;
                    if let Some(intermediate) = intermedidate {
                        self.intermediate.push_operand(intermediate);
                    }

                    (instance, is_mutable)
                } else {
                    (*instance, true)
                };

                self.evaluation_stack.push(StackElement::Evaluated(result));
                self.next_call_type = CallType::new_method(instance, is_mutable);

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
    ) -> Result<GeneratorExpressionElement, Error> {
        let call_type = self.next_call_type.take();

        let (operand_2, _intermediate_2) =
            Self::evaluate(self.scope_stack.top(), self.evaluation_stack.pop(), rule)?;
        let (operand_1, _intermediate_1) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Type,
        )?;

        let (element, intermediate) = CallAnalyzer::analyze(
            self.scope_stack.top(),
            operand_1,
            operand_2,
            call_type,
            location,
        )?;

        self.evaluation_stack.push(StackElement::Evaluated(element));

        Ok(intermediate)
    }

    ///
    /// Analyzes the path resolution operation.
    ///
    fn path(&mut self) -> Result<(), Error> {
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

        let result = Element::path(operand_1, operand_2)?;
        self.evaluation_stack.push(StackElement::Evaluated(result));

        Ok(())
    }

    ///
    /// Analyzes the structure initialization operation.
    ///
    fn structure(&mut self) -> Result<(), Error> {
        let (operand_2, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            self.rule,
        )?;
        let (operand_1, _) = Self::evaluate(
            self.scope_stack.top(),
            self.evaluation_stack.pop(),
            TranslationRule::Type,
        )?;

        let result = Element::structure(operand_1, operand_2, self.scope_stack.top())?;
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
        match element {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::LiteralUnit(location) => Ok((
                    Element::Constant(Constant::Unit(UnitConstant::new(location))),
                    None,
                )),
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
                ExpressionOperand::List(inner) => ListAnalyzer::analyze(scope, inner, rule),
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
