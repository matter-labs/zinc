//!
//! The `for` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#for::Statement as ForStatement;

///
/// The `for` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The loop index variable identifier.
    index_identifier: Option<Identifier>,
    /// The loop index bounds range expression.
    bounds_expression: Option<ExpressionTree>,
    /// The optional loop `while` condition expression.
    while_condition: Option<ExpressionTree>,
    /// The loop block.
    block: Option<BlockExpression>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_index_identifier(&mut self, value: Identifier) {
        self.index_identifier = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_bounds_expression(&mut self, value: ExpressionTree) {
        self.bounds_expression = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_while_condition(&mut self, value: ExpressionTree) {
        self.while_condition = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_block(&mut self, value: BlockExpression) {
        self.block = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ForStatement {
        ForStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.index_identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "index identifier"
                )
            }),
            self.bounds_expression.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "bounds expression"
                )
            }),
            self.while_condition.take(),
            self.block.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "block")
            }),
        )
    }
}
