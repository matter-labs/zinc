//!
//! The attribute.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

///
/// The attribute.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// The location of the syntax construction.
    pub location: Location,
    /// If the attribute is related to the enclosing item, e.g. a module or block.
    pub is_inner: bool,
    /// The attribute identifier.
    pub identifier: Identifier,
}

impl Attribute {
    ///
    /// Creates the attribute value.
    ///
    pub fn new(location: Location, is_inner: bool, identifier: Identifier) -> Self {
        Self {
            location,
            is_inner,
            identifier,
        }
    }
}
