//!
//! The `match` expression semantic analyzer.
//!

#[cfg(test)]
mod tests;

pub mod exhausting;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use zinc_syntax::MatchExpression;
use zinc_syntax::MatchPatternVariant;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::r#match::builder::Builder as GeneratorMatchExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::r#type::Type as GeneratorType;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::casting::Caster;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::unit::Unit as UnitConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

use self::exhausting::Data as ExhaustingData;

///
/// The `match` expression semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    /// The number of mandatory branches number to generate useful conditional code.
    const REQUIRED_BRANCHES_COUNT: usize = 2;

    ///
    /// Analyzes the match expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        r#match: MatchExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, r#match).map(|element| (element, None))
            }
            _rule => Self::runtime(scope, r#match)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime match semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        r#match: MatchExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = r#match.location;

        let mut builder = GeneratorMatchExpressionBuilder::default();

        builder.set_location(r#match.location);

        let mut scope_stack = ScopeStack::new(scope);

        let scrutinee_location = r#match.scrutinee.location;
        let (scrutinee_result, scrutinee_expression) =
            ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                .analyze(r#match.scrutinee)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, scope_stack.top())?;
        if scrutinee_type.is_scalar() {
            builder.set_scrutinee(
                scrutinee_expression,
                GeneratorType::try_from_semantic(&scrutinee_type)
                    .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
            );
        } else {
            return Err(Error::MatchScrutineeInvalidType {
                location: scrutinee_location,
                found: scrutinee_type.to_string(),
            });
        }

        if r#match.branches.len() < Self::REQUIRED_BRANCHES_COUNT {
            return Err(Error::MatchLessThanTwoBranches { location });
        }

        let first_branch_expression_location = r#match.branches[0].1.location;
        let mut is_exhausted = false;
        let mut exhausting_data = ExhaustingData::new();
        let mut match_result = None;

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
                    let location = boolean.location;

                    let constant = BooleanConstant::from(boolean);
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    if let Some(duplicate) =
                        exhausting_data.insert_boolean(constant.inner, location)
                    {
                        return Err(Error::MatchBranchDuplicate {
                            location,
                            reference: duplicate,
                        });
                    }

                    let constant =
                        GeneratorConstant::try_from_semantic(&Constant::Boolean(constant))
                            .expect(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(expression)?;

                    if exhausting_data.has_exhausted_boolean() {
                        is_exhausted = true;
                        builder.set_wildcard_branch(branch);
                    } else {
                        builder.push_branch(constant, branch);
                    }

                    result
                }
                MatchPatternVariant::IntegerLiteral(integer) => {
                    let location = integer.location;

                    let constant = IntegerConstant::try_from(&integer)?;
                    let pattern_type = constant.r#type();
                    if Caster::cast(&pattern_type, &scrutinee_type).is_err() {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    if let Some(duplicate) =
                        exhausting_data.insert_integer(constant.value.clone(), None, location)
                    {
                        return Err(Error::MatchBranchDuplicate {
                            location,
                            reference: duplicate,
                        });
                    }

                    let constant =
                        GeneratorConstant::try_from_semantic(&Constant::Integer(constant))
                            .expect(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(expression)?;

                    if exhausting_data.has_exhausted_integer() {
                        is_exhausted = true;
                        builder.set_wildcard_branch(branch);
                    } else {
                        builder.push_branch(constant, branch);
                    }

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    let constant =
                        match ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(path)?
                        {
                            (Element::Constant(constant), _intermediate) => {
                                if let Constant::Integer(ref integer) = constant {
                                    if let Some(duplicate) = exhausting_data.insert_integer(
                                        integer.value.to_owned(),
                                        integer.enumeration.to_owned(),
                                        location,
                                    ) {
                                        return Err(Error::MatchBranchDuplicate {
                                            location,
                                            reference: duplicate,
                                        });
                                    }
                                }
                                constant
                            }
                            (element, _intermediate) => {
                                return Err(Error::MatchBranchPatternPathExpectedConstant {
                                    location,
                                    found: element.to_string(),
                                });
                            }
                        };
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
                        .expect(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);
                    let (result, branch) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(expression)?;

                    if exhausting_data.has_exhausted_integer() {
                        is_exhausted = true;
                        builder.set_wildcard_branch(branch);
                    } else {
                        builder.push_branch(constant, branch);
                    }

                    result
                }
                MatchPatternVariant::Binding(identifier) => {
                    is_exhausted = true;

                    scope_stack.push(None, ScopeType::Block);
                    Scope::define_variable(
                        scope_stack.top(),
                        identifier.clone(),
                        false,
                        scrutinee_type.clone(),
                    )?;
                    let (result, branch) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(expression)?;
                    scope_stack.pop();

                    builder.set_binding_branch(branch, identifier.name);

                    result
                }
                MatchPatternVariant::Wildcard => {
                    is_exhausted = true;
                    let (result, branch) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                            .analyze(expression)?;

                    builder.set_wildcard_branch(branch);

                    result
                }
            };

            let result_type = Type::from_element(&result, scope_stack.top())?;
            if let Some(ref match_result) = match_result {
                let match_result_type = Type::from_element(match_result, scope_stack.top())?;
                if result_type != match_result_type {
                    return Err(Error::MatchBranchExpressionInvalidType {
                        location: expression_location,
                        expected: match_result_type.to_string(),
                        found: result_type.to_string(),
                        reference: first_branch_expression_location,
                    });
                }
            }

            if match_result.is_none() {
                match_result = Some(result);
            }
        }

        if !is_exhausted {
            return Err(Error::MatchNotExhausted { location });
        }

        let element = match match_result.take() {
            Some(result) => result,
            None => Element::Constant(Constant::Unit(UnitConstant::new(location))),
        };

        let intermediate = GeneratorExpressionOperand::Match(builder.finish());

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant match semantic element.
    ///
    fn constant(scope: Rc<RefCell<Scope>>, r#match: MatchExpression) -> Result<Element, Error> {
        let location = r#match.location;

        let mut scope_stack = ScopeStack::new(scope);

        let scrutinee_location = r#match.scrutinee.location;
        let (scrutinee_result, _) =
            ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                .analyze(r#match.scrutinee)?;
        let scrutinee_result = match scrutinee_result {
            Element::Constant(constant) => constant,
            element => {
                return Err(Error::ExpressionNonConstantElement {
                    location: scrutinee_location,
                    found: element.to_string(),
                });
            }
        };
        let scrutinee_type = scrutinee_result.r#type();
        if !scrutinee_type.is_scalar() {
            return Err(Error::MatchScrutineeInvalidType {
                location: scrutinee_location,
                found: scrutinee_type.to_string(),
            });
        }

        if r#match.branches.len() < Self::REQUIRED_BRANCHES_COUNT {
            return Err(Error::MatchLessThanTwoBranches { location });
        }

        let first_branch_expression_location = r#match.branches[0].1.location;
        let mut is_exhausted = false;
        let mut exhausting_data = ExhaustingData::new();
        let mut match_result = None;

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
                    let location = boolean.location;

                    let constant = BooleanConstant::from(boolean);
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    if let Some(duplicate) =
                        exhausting_data.insert_boolean(constant.inner, location)
                    {
                        return Err(Error::MatchBranchDuplicate {
                            location,
                            reference: duplicate,
                        });
                    }

                    let expression_location = expression.location;
                    let (result, _) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(expression)?;
                    match result {
                        Element::Constant(ref result) => {
                            if Constant::Boolean(constant) == scrutinee_result {
                                match_result = Some(result.to_owned());
                            }
                        }
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }

                    if exhausting_data.has_exhausted_boolean() {
                        is_exhausted = true;
                    }

                    result
                }
                MatchPatternVariant::IntegerLiteral(integer) => {
                    let location = integer.location;

                    let constant = IntegerConstant::try_from(&integer)?;
                    let pattern_type = constant.r#type();
                    if Caster::cast(&pattern_type, &scrutinee_type).is_err() {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    if let Some(duplicate) =
                        exhausting_data.insert_integer(constant.value.clone(), None, location)
                    {
                        return Err(Error::MatchBranchDuplicate {
                            location,
                            reference: duplicate,
                        });
                    }

                    let expression_location = expression.location;
                    let (result, _) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(expression)?;
                    match result {
                        Element::Constant(ref result) => {
                            if Constant::Integer(constant) == scrutinee_result {
                                match_result = Some(result.to_owned());
                            }
                        }
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }

                    if exhausting_data.has_exhausted_integer() {
                        is_exhausted = true;
                    }

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    let constant =
                        match ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(path)?
                        {
                            (Element::Constant(constant), _intermediate) => {
                                if let Constant::Integer(ref integer) = constant {
                                    if let Some(duplicate) = exhausting_data.insert_integer(
                                        integer.value.to_owned(),
                                        integer.enumeration.to_owned(),
                                        location,
                                    ) {
                                        return Err(Error::MatchBranchDuplicate {
                                            location,
                                            reference: duplicate,
                                        });
                                    }
                                }
                                constant
                            }
                            (element, _intermediate) => {
                                return Err(Error::MatchBranchPatternPathExpectedConstant {
                                    location,
                                    found: element.to_string(),
                                });
                            }
                        };
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType {
                            location: pattern_location,
                            expected: scrutinee_type.to_string(),
                            found: pattern_type.to_string(),
                            reference: scrutinee_location,
                        });
                    }

                    let expression_location = expression.location;
                    let (result, _) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(expression)?;
                    match result {
                        Element::Constant(ref result) => {
                            if constant == scrutinee_result {
                                match_result = Some(result.to_owned());
                            }
                        }
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }

                    if exhausting_data.has_exhausted_integer() {
                        is_exhausted = true;
                    }

                    result
                }
                MatchPatternVariant::Binding(identifier) => {
                    is_exhausted = true;

                    scope_stack.push(None, ScopeType::Block);
                    Scope::define_constant(
                        scope_stack.top(),
                        identifier.clone(),
                        scrutinee_result.clone(),
                    )?;
                    let expression_location = expression.location;
                    let (result, _) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(expression)?;
                    scope_stack.pop();

                    match result {
                        Element::Constant(ref result) => {
                            if match_result.is_none() {
                                match_result = Some(result.to_owned());
                            }
                        }
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }

                    result
                }
                MatchPatternVariant::Wildcard => {
                    is_exhausted = true;
                    let expression_location = expression.location;
                    let (result, _) =
                        ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                            .analyze(expression)?;

                    match result {
                        Element::Constant(ref result) => {
                            if match_result.is_none() {
                                match_result = Some(result.to_owned());
                            }
                        }
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }

                    result
                }
            };

            let result_type = Type::from_element(&result, scope_stack.top())?;
            if let Some(ref match_result) = match_result {
                let match_result_type = match_result.r#type();
                if result_type != match_result_type {
                    return Err(Error::MatchBranchExpressionInvalidType {
                        location: expression_location,
                        expected: match_result_type.to_string(),
                        found: result_type.to_string(),
                        reference: first_branch_expression_location,
                    });
                }
            }
        }

        if !is_exhausted {
            return Err(Error::MatchNotExhausted { location });
        }

        let element = Element::Constant(match match_result.take() {
            Some(result) => result,
            None => Constant::Unit(UnitConstant::new(location)),
        });

        Ok(element)
    }
}
