//!
//! The tuple index builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::tuple_index::TupleIndex;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    literal: Option<IntegerLiteral>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_literal(&mut self, value: IntegerLiteral) {
        self.literal = Some(value);
    }

    pub fn finish(mut self) -> TupleIndex {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location"));

        let literal = self
            .literal
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "literal"));

        TupleIndex::new(location, literal)
    }
}
