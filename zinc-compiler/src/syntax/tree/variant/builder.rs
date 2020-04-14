//!
//! The variant builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::variant::Variant;

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
            self.location.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.literal
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "literal")),
        )
    }
}
