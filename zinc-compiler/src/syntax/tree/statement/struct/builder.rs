//!
//! The struct statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Field;
use crate::syntax::Identifier;
use crate::syntax::StructStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    fields: Vec<Field>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_fields(&mut self, value: Vec<Field>) {
        self.fields = value;
    }

    pub fn finish(mut self) -> StructStatement {
        StructStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.fields,
        )
    }
}
