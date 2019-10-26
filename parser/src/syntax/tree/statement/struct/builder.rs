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
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.fields,
        )
    }
}
