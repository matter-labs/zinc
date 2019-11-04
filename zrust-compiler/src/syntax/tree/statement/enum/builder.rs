//!
//! The struct statement builder.
//!

use crate::lexical::Location;
use crate::syntax::EnumStatement;
use crate::syntax::Identifier;
use crate::syntax::Variant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    variants: Vec<Variant>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_variants(&mut self, value: Vec<Variant>) {
        self.variants = value;
    }

    pub fn finish(mut self) -> EnumStatement {
        EnumStatement::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.variants,
        )
    }
}
