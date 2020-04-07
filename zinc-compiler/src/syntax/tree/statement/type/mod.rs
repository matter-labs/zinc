//!
//! The type statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Statement {
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
