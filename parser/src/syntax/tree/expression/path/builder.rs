//!
//! The path expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::PathExpression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<Identifier>,
}

impl Builder {
    pub fn set_location_if_unset(&mut self, value: Location) {
        if self.location.is_none() {
            self.location = Some(value);
        }
    }

    pub fn push_identifier(&mut self, identifier: Identifier) {
        self.elements.push(identifier);
    }

    pub fn finish(mut self) -> PathExpression {
        PathExpression::new(
            self.location.take().expect("Missing location"),
            self.elements,
        )
    }
}
