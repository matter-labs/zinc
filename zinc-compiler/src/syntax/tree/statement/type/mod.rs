//!
//! The type statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: syntax::Type,
}

impl Statement {
    pub fn new(location: Location, identifier: Identifier, r#type: syntax::Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
