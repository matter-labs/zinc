//!
//! The attribute builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::identifier::Identifier;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_inner: bool,
    identifier: Option<Identifier>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_inner(&mut self) {
        self.is_inner = true;
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn finish(mut self) -> Attribute {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        let identifier = self
            .identifier
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier"));

        Attribute::new(location, self.is_inner, identifier)
    }
}
