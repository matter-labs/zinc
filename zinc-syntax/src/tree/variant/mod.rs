//!
//! The enumeration variant.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::literal::integer::Literal as IntegerLiteral;

///
/// The enumeration variant.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The location of the syntax construction.
    pub location: Location,
    /// The enumeration variant identifier.
    pub identifier: Identifier,
    /// The enumeration variant integer value.
    pub literal: IntegerLiteral,
}

impl Variant {
    ///
    /// Creates an enumeration variant.
    ///
    pub fn new(location: Location, identifier: Identifier, literal: IntegerLiteral) -> Self {
        Self {
            location,
            identifier,
            literal,
        }
    }
}
