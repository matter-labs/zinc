//!
//! The struct statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::field::Field;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub fields: Vec<Field>,
}

impl Statement {
    pub fn new(location: Location, identifier: Identifier, fields: Vec<Field>) -> Self {
        Self {
            location,
            identifier,
            fields,
        }
    }
}
