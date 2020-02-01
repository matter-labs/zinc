//!
//! The loop statement builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::LoopStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    index_identifier: Option<Identifier>,
    bounds_expression: Option<Expression>,
    while_condition: Option<Expression>,
    block: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_index_identifier(&mut self, value: Identifier) {
        self.index_identifier = Some(value);
    }

    pub fn set_bounds_expression(&mut self, value: Expression) {
        self.bounds_expression = Some(value);
    }

    pub fn set_while_condition(&mut self, value: Expression) {
        self.while_condition = Some(value);
    }

    pub fn set_block(&mut self, value: BlockExpression) {
        self.block = Some(value);
    }

    pub fn finish(mut self) -> LoopStatement {
        LoopStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.index_identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "index identifier"
                )
            }),
            self.bounds_expression.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "range start expression"
                )
            }),
            self.while_condition.take(),
            self.block.take().unwrap_or_else(|| {
                panic!("{}{}", crate::syntax::PANIC_BUILDER_REQUIRES_VALUE, "block")
            }),
        )
    }
}
