//!
//! The match expression semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::r#match::builder::Builder as GeneratorMatchExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::r#type::Type as GeneratorType;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::expression::r#match::Expression as MatchExpression;
use crate::syntax::tree::pattern_match::variant::Variant as MatchPatternVariant;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the match expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        r#match: MatchExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = r#match.location;

        let mut builder = GeneratorMatchExpressionBuilder::default();

        builder.set_location(r#match.location);

        let mut scope_stack = ScopeStack::new(scope);

        let scrutinee_location = r#match.scrutinee.location;
        let (scrutinee_result, scrutinee_expression) =
            ExpressionAnalyzer::new(scope_stack.top())
                .analyze(r#match.scrutinee, TranslationHint::Value)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, scope_stack.top())?;
        if scrutinee_type.is_scalar() {
            builder.set_scrutinee(
                scrutinee_expression,
                GeneratorType::try_from_semantic(&scrutinee_type)
                    .expect(crate::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
            );
        } else {
            return Err(Error::MatchScrutineeInvalidType {
                location: scrutinee_location,
                found: scrutinee_type.to_string(),
            });
        }

        if r#match.branches.len() < 2 {
            return Err(Error::MatchLessThanTwoBranches { location });
        }

        let first_branch_expression_location = r#match.branches[0].1.location;
        let mut is_exhausted = false;
        let mut branch_results = Vec::with_capacity(r#match.branches.len());

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
                        .expect(crate::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) = ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(expression, TranslationHint::Value)?;
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
                        .expect(crate::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) = ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(expression, TranslationHint::Value)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    let constant = match ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(path, TranslationHint::Value)?
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
                        .expect(crate::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) = ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(expression, TranslationHint::Value)?;
                    builder.push_branch(constant, branch);

                    result
                }
                MatchPatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    scope_stack.push();
                    Scope::declare_variable(
                        scope_stack.top(),
                        identifier.clone(),
                        ScopeVariableItem::new(false, scrutinee_type.clone()),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                    let (result, branch) = ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(expression, TranslationHint::Value)?;
                    builder.set_binding_branch(branch, identifier.name);
                    scope_stack.pop();

                    result
                }
                MatchPatternVariant::Wildcard => {
                    is_exhausted = true;
                    let (result, branch) = ExpressionAnalyzer::new(scope_stack.top())
                        .analyze(expression, TranslationHint::Value)?;
                    builder.set_wildcard_branch(branch);
                    result
                }
            };

            let result_type = Type::from_element(&result, scope_stack.top())?;
            if let Some(first_branch_result) = branch_results.get(0) {
                let first_branch_result_type =
                    Type::from_element(first_branch_result, scope_stack.top())?;
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
        let intermediate = GeneratorExpressionOperand::Match(builder.finish());

        Ok((element, intermediate))
    }
}
