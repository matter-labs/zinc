//!
//! The module statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
}

impl Statement {
    pub fn new(location: Location, identifier: Identifier) -> Self {
        Self {
            location,
            identifier,
        }
    }
}
