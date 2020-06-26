//!
//! The `mod` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

///
/// The `mod` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The module identifier.
    pub identifier: Identifier,
}

impl Statement {
    ///
    /// Creates a `mod` statement.
    ///
    pub fn new(location: Location, identifier: Identifier) -> Self {
        Self {
            location,
            identifier,
        }
    }
}
