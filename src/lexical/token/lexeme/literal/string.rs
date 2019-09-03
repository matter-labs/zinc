//!
//! The string literal lexeme.
//!

use std::fmt;
use std::str;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "value")]
pub struct String {
    pub value: Vec<u8>,
}

impl String {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.value) })
    }
}
