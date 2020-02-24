//!
//! The member string.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberString {
    pub location: Location,
    pub name: String,
}

impl MemberString {
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }
}

impl From<Identifier> for MemberString {
    fn from(identifier: Identifier) -> Self {
        Self {
            location: identifier.location,
            name: identifier.name,
        }
    }
}
