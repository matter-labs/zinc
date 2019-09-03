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
    pub is_mutable: bool,
}

impl Place {
    pub fn new(identifier: Identifier, value: Value, is_mutable: bool) -> Self {
        Self {
            identifier,
            value,
            is_mutable,
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} = {}",
            if self.is_mutable { "mut " } else { "" },
            self.identifier,
            self.value
        )
    }
}
