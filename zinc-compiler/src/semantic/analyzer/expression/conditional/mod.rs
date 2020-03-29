//!
//! The conditional expression semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::conditional::builder::Builder as GeneratorConditionalExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::ConditionalExpression;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
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

        let mut scope_stack = ScopeStack::new(scope);

        let (condition_result, condition) = ExpressionAnalyzer::new(scope_stack.top())
            .analyze(*conditional.condition, TranslationHint::ValueExpression)?;
        match Type::from_element(&condition_result, scope_stack.top())? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition {
                    location: condition_location,
                    found: r#type.to_string(),
                });
            }
        }
        builder.set_condition(condition);

        scope_stack.push();
        let (main_result, main_block) =
            BlockAnalyzer::analyze(scope_stack.top(), conditional.main_block)?;
        let main_type = Type::from_element(&main_result, scope_stack.top())?;
        scope_stack.pop();
        builder.set_main_block(main_block);

        let else_type = if let Some(else_block) = conditional.else_block {
            scope_stack.push();
            let (else_result, else_block) = BlockAnalyzer::analyze(scope_stack.top(), else_block)?;
            let else_type = Type::from_element(&else_result, scope_stack.top())?;
            scope_stack.pop();
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

        let element = main_result;
        let intermediate = GeneratorExpressionOperand::Conditional(builder.finish());

        Ok((element, intermediate))
    }
}
