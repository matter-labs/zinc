//!
//! The `for` statement semantic analyzer.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::rc::Rc;

use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::generator::statement::r#for::Statement as GeneratorForLoopStatement;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#for::error::Error as ForStatementError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::memory_type::MemoryType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#for::Statement as ForStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a for-loop and returns its IR for the next compiler phase.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        statement: ForStatement,
    ) -> Result<GeneratorForLoopStatement, Error> {
        let location = statement.location;
        let bounds_expression_location = statement.bounds_expression.location;

        let mut scope_stack = ScopeStack::new(scope);

        let (range_start, range_end, index_bitlength, is_index_signed, is_inclusive) =
            match ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Constant)
                .analyze(statement.bounds_expression)?
            {
                (Element::Constant(Constant::RangeInclusive(range)), _intermediate) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    true,
                ),
                (Element::Constant(Constant::Range(range)), _intermediate) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    false,
                ),
                (element, _intermediate) => {
                    return Err(Error::Statement(StatementError::For(
                        ForStatementError::BoundsExpectedConstantRangeExpression {
                            location: bounds_expression_location,
                            found: element.to_string(),
                        },
                    )));
                }
            };

        scope_stack.push(None);

        let index_location = statement.index_identifier.location;
        let index_identifier = statement.index_identifier.name.to_owned();
        Scope::define_variable(
            scope_stack.top(),
            statement.index_identifier,
            false,
            Type::scalar(Some(index_location), is_index_signed, index_bitlength),
            MemoryType::Stack,
        )?;

        let while_condition = if let Some(expression) = statement.while_condition {
            let location = expression.location;
            let (while_result, while_intermediate) =
                ExpressionAnalyzer::new(scope_stack.top(), TranslationRule::Value)
                    .analyze(expression)?;

            match Type::from_element(&while_result, scope_stack.top())? {
                Type::Boolean(_) => {}
                r#type => {
                    return Err(Error::Statement(StatementError::For(
                        ForStatementError::WhileExpectedBooleanCondition {
                            location,
                            found: r#type.to_string(),
                        },
                    )));
                }
            }

            Some(while_intermediate)
        } else {
            None
        };

        let (_element, body) =
            BlockAnalyzer::analyze(scope_stack.top(), statement.block, TranslationRule::Value)?;

        scope_stack.pop();

        let is_reversed = range_start > range_end;

        let iterations_count = (range_end - range_start.clone()).abs();
        let mut iterations_count =
            iterations_count
                .to_usize()
                .ok_or(Error::Element(ElementError::Constant(
                    ConstantError::Integer(IntegerConstantError::IntegerTooLarge {
                        location: bounds_expression_location,
                        value: iterations_count,
                        bitlength: index_bitlength,
                    }),
                )))?;
        if is_inclusive {
            iterations_count += 1;
        }

        Ok(GeneratorForLoopStatement::new(
            location,
            range_start,
            iterations_count,
            is_reversed,
            index_identifier,
            is_index_signed,
            index_bitlength,
            while_condition,
            body,
        ))
    }
}
