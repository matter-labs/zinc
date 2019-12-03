//!
//! The loop statement builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::LoopStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    index_identifier: Option<Identifier>,
    range_start: Option<IntegerLiteral>,
    range_end: Option<IntegerLiteral>,
    is_range_inclusive: bool,
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

    pub fn set_range_start(&mut self, value: IntegerLiteral) {
        self.range_start = Some(value);
    }

    pub fn set_range_end(&mut self, value: IntegerLiteral) {
        self.range_end = Some(value);
    }

    pub fn set_range_inclusive(&mut self) {
        self.is_range_inclusive = true;
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
            self.range_start.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "range start"
                )
            }),
            self.range_end.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "range end"
                )
            }),
            self.is_range_inclusive,
            self.while_condition.take(),
            self.block.take().unwrap_or_else(|| {
                panic!("{}{}", crate::syntax::PANIC_BUILDER_REQUIRES_VALUE, "block")
            }),
        )
    }
}
