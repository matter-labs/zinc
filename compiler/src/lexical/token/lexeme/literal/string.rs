//!
//! The string literal lexeme.
//!

use std::fmt;
use std::string;

#[derive(Debug, Clone, PartialEq)]
pub struct String {
    pub value: string::String,
}

impl String {
    pub fn new(value: string::String) -> Self {
        Self { value }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
