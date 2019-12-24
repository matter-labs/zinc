//!
//! The member integer builder.
//!

use crate::lexical::Location;
use crate::syntax::IntegerLiteral;
use crate::syntax::MemberInteger;

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

    pub fn finish(mut self) -> MemberInteger {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let literal = self.literal.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "literal"
            )
        });

        MemberInteger::new(location, literal)
    }
}
