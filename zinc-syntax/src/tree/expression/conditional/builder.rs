//!
//! The conditional expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::conditional::Expression as ConditionalExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The conditional expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The condition expression.
    condition: Option<ExpressionTree>,
    /// The main conditional block expression.
    main_block: Option<BlockExpression>,
    /// The `else` conditional block expression.
    else_block: Option<BlockExpression>,
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
    pub fn set_condition(&mut self, value: ExpressionTree) {
        self.condition = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_main_block(&mut self, value: BlockExpression) {
        self.main_block = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_else_block(&mut self, value: BlockExpression) {
        self.else_block = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ConditionalExpression {
        ConditionalExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.condition.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "condition"
                )
            }),
            self.main_block.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "main block"
                )
            }),
            self.else_block.take(),
        )
    }
}
