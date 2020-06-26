//!
//! The structure field.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

///
/// The structure field.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The location of the syntax construction.
    pub location: Location,
    /// The structure field identifier.
    pub identifier: Identifier,
    /// The structure field type.
    pub r#type: Type,
}

impl Field {
    ///
    /// Creates a structure field.
    ///
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
