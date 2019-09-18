//!
//! The witness.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Witness {
    #[serde(skip_serializing)]
    location: Location,
    identifier: Identifier,
    r#type: Type,
}

impl Witness {
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn r#type(&self) -> &Type {
        &self.r#type
    }
}
