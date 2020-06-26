//!
//! The `enum` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::variant::Variant;

///
/// The `enum` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The enumeration type identifier.
    pub identifier: Identifier,
    /// The enumeration type variants.
    pub variants: Vec<Variant>,
}

impl Statement {
    ///
    /// Creates an `enum` statement.
    ///
    pub fn new(location: Location, identifier: Identifier, variants: Vec<Variant>) -> Self {
        Self {
            location,
            identifier,
            variants,
        }
    }
}
