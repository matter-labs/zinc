//!
//! The member string.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;

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
