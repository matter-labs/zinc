//!
//! The `type` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::r#type::Type;

///
/// The `type` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The type alias identifier.
    pub identifier: Identifier,
    /// The type alias expression.
    pub r#type: Type,
}

impl Statement {
    ///
    /// Creates a `type` statement.
    ///
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
