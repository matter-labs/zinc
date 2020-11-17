//!
//! The conditional expression semantic analyzer.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ConditionalExpression;

use crate::generator::expression::operand::conditional::builder::Builder as GeneratorConditionalExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::unit::Unit as UnitConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

///
/// The conditional expression semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the conditional expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        conditional: ConditionalExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, conditional).map(|element| (element, None))
            }
            _rule => Self::runtime(scope, conditional)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime conditional semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        conditional: ConditionalExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
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

        builder.set_location(conditional.location);

        let mut scope_stack = ScopeStack::new(scope);

        let (condition_result, condition) =
            ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                .analyze(*conditional.condition)?;
        match Type::from_element(&condition_result, scope_stack.top())? {
            Type::Boolean(_) => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition {
                    location: condition_location,
                    found: r#type.to_string(),
                });
            }
        }
        builder.set_condition(condition);

        scope_stack.push(None, ScopeType::Conditional);
        let (main_result, main_block) = BlockAnalyzer::analyze(
            scope_stack.top(),
            conditional.main_block,
            TranslationRule::Value,
        )?;
        let main_type = Type::from_element(&main_result, scope_stack.top())?;
        scope_stack.pop();
        builder.set_main_block(main_block);

        let else_type = if let Some(else_block) = conditional.else_block {
            scope_stack.push(None, ScopeType::Conditional);
            let (else_result, else_block) =
                BlockAnalyzer::analyze(scope_stack.top(), else_block, TranslationRule::Value)?;
            let else_type = Type::from_element(&else_result, scope_stack.top())?;
            scope_stack.pop();
            builder.set_else_block(else_block);

            else_type
        } else {
            Type::unit(None)
        };

        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch {
                location: main_expression_location,
                expected: main_type.to_string(),
                found: else_type.to_string(),
                reference: else_expression_location,
            });
        }

        let element = main_result;

        let intermediate = GeneratorExpressionOperand::Conditional(builder.finish());

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant conditional semantic element.
    ///
    fn constant(
        scope: Rc<RefCell<Scope>>,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        let location = conditional.location;

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

        let mut scope_stack = ScopeStack::new(scope);

        let (condition_result, _) =
            ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                .analyze(*conditional.condition)?;
        let condition_result = match condition_result {
            Element::Constant(constant) => constant,
            element => {
                return Err(Error::ExpressionNonConstantElement {
                    location: condition_location,
                    found: element.to_string(),
                });
            }
        };
        let condition_result = match condition_result {
            Constant::Boolean(boolean) => boolean,
            constant => {
                return Err(Error::ConditionalExpectedBooleanCondition {
                    location: condition_location,
                    found: constant.r#type().to_string(),
                });
            }
        };

        scope_stack.push(None, ScopeType::Conditional);
        let (main_result, _) = BlockAnalyzer::analyze(
            scope_stack.top(),
            conditional.main_block,
            TranslationRule::Constant,
        )?;
        let main_result = match main_result {
            Element::Constant(constant) => constant,
            element => {
                return Err(Error::ExpressionNonConstantElement {
                    location: main_expression_location,
                    found: element.to_string(),
                });
            }
        };
        let main_type = main_result.r#type();
        scope_stack.pop();

        let (else_type, else_result) = if let Some(else_block) = conditional.else_block {
            scope_stack.push(None, ScopeType::Conditional);
            let (else_result, _) =
                BlockAnalyzer::analyze(scope_stack.top(), else_block, TranslationRule::Constant)?;
            let else_result = match else_result {
                Element::Constant(constant) => constant,
                element => {
                    return Err(Error::ExpressionNonConstantElement {
                        location: else_expression_location,
                        found: element.to_string(),
                    });
                }
            };
            let else_type = else_result.r#type();
            scope_stack.pop();

            (else_type, else_result)
        } else {
            (
                Type::unit(None),
                Constant::Unit(UnitConstant::new(location)),
            )
        };

        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch {
                location: main_expression_location,
                expected: main_type.to_string(),
                found: else_type.to_string(),
                reference: else_expression_location,
            });
        }

        let element = Element::Constant(if condition_result.is_true() {
            main_result
        } else {
            else_result
        });

        Ok(element)
    }
}
