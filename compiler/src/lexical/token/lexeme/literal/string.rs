//!
//! The string literal lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "value")]
pub struct String {
    pub value: std::string::String,
}

impl String {
    pub fn new(value: std::string::String) -> Self {
        Self { value }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
