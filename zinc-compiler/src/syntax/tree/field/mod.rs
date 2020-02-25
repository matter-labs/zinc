//!
//! The field.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

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
