//!
//! The conditional expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::conditional::Expression as ConditionalExpression;
use crate::syntax::tree::expression::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    condition: Option<Expression>,
    main_block: Option<BlockExpression>,
    else_block: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_condition(&mut self, value: Expression) {
        self.condition = Some(value);
    }

    pub fn set_main_block(&mut self, value: BlockExpression) {
        self.main_block = Some(value);
    }

    pub fn set_else_block(&mut self, value: BlockExpression) {
        self.else_block = Some(value);
    }

    pub fn finish(mut self) -> ConditionalExpression {
        ConditionalExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.condition.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "condition"
                )
            }),
            self.main_block.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "main block"
                )
            }),
            self.else_block.take(),
        )
    }
}
