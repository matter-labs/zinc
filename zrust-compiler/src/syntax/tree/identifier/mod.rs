//!
//! The identifier.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub location: Location,
    pub name: String,
    pub is_instruction: bool,
}

impl Identifier {
    pub fn new(location: Location, name: String, is_instruction: bool) -> Self {
        Self { location, name, is_instruction }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
