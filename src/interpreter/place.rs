//!
//! The interpreter place.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::interpreter::Value;
use crate::syntax::Identifier;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Place {
    pub identifier: Identifier,
    pub value: Value,
}

impl Place {
    pub fn new(identifier: Identifier, value: Value) -> Self {
        Self { identifier, value }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.identifier, self.value)
    }
}
