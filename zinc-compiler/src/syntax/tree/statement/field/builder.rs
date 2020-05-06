//!
//! The field statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;
use crate::syntax::tree::statement::field::Statement as FieldStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_public: bool,
    identifier: Option<Identifier>,
    r#type: Option<Type>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_is_public(&mut self) {
        self.is_public = true;
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn finish(mut self) -> FieldStatement {
        FieldStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.is_public,
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.r#type
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "type")),
        )
    }
}
