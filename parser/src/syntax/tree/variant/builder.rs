//!
//! The variant builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Variant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    literal: Option<IntegerLiteral>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_literal(&mut self, value: IntegerLiteral) {
        self.literal = Some(value);
    }

    pub fn finish(&mut self) -> Variant {
        Variant::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.literal.take().expect("Missing literal"),
        )
    }
}
