//!
//! The field.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Field {
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
