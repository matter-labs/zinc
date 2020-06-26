//!
//! The `struct` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::field::Field;
use crate::syntax::tree::identifier::Identifier;

///
/// The `struct` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The structure type identifier.
    pub identifier: Identifier,
    /// The structure type fields.
    pub fields: Vec<Field>,
}

impl Statement {
    ///
    /// Creates a `struct` statement.
    ///
    pub fn new(location: Location, identifier: Identifier, fields: Vec<Field>) -> Self {
        Self {
            location,
            identifier,
            fields,
        }
    }
}
