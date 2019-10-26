//!
//! The field builder.
//!

use crate::lexical::Location;
use crate::syntax::Field;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    r#type: Option<Type>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn finish(&mut self) -> Field {
        Field::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.r#type.take().expect("Missing type"),
        )
    }
}
