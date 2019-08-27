//!
//! The identifier.
//!

use std::fmt;
use std::str;

use serde_derive::Serialize;

use crate::lexical::Location;

#[derive(Debug, Serialize, PartialEq)]
pub struct Identifier {
    pub location: Location,
    pub name: Vec<u8>,
}

impl Identifier {
    pub fn new(location: Location, name: Vec<u8>) -> Self {
        Self { location, name }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.name) })
    }
}
