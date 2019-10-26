//!
//! The module statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::ModStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn finish(mut self) -> ModStatement {
        ModStatement::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
        )
    }
}
