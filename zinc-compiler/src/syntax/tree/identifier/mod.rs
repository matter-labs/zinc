//!
//! The identifier.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::member_string::MemberString;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub location: Location,
    pub name: String,
}

impl Identifier {
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }
}

impl From<MemberString> for Identifier {
    fn from(member_string: MemberString) -> Self {
        Self {
            location: member_string.location,
            name: member_string.name,
        }
    }
}
