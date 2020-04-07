//!
//! The enum statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;
use crate::syntax::tree::variant::Variant;

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
            self.location
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location")),
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.variants,
        )
    }
}
