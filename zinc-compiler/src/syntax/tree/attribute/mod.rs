//!
//! The attribute.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub location: Location,
    pub is_inner: bool,
    pub identifier: Identifier,
}

impl Attribute {
    pub fn new(location: Location, is_inner: bool, identifier: Identifier) -> Self {
        Self {
            location,
            is_inner,
            identifier,
        }
    }
}
