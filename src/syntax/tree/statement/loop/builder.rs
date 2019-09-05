//!
//! The loop statement builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Identifier;
use crate::syntax::Loop;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    index_identifier: Option<Identifier>,
    range_start: Option<IntegerLiteral>,
    range_end: Option<IntegerLiteral>,
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

    pub fn set_block(&mut self, value: BlockExpression) {
        self.block = Some(value);
    }

    pub fn finish(mut self) -> Loop {
        Loop::new(
            self.location.take().expect("Missing location"),
            self.index_identifier
                .take()
                .expect("Missing index identifier"),
            self.range_start.take().expect("Missing range start"),
            self.range_end.take().expect("Missing range end"),
            self.block.take().expect("Missing block"),
        )
    }
}
