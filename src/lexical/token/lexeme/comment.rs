//!
//! The comment lexeme.
//!

use std::fmt;
use std::str;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Comment {
    pub text: Vec<u8>,
}

impl Comment {
    pub fn new(text: Vec<u8>) -> Self {
        Self { text }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.text) })
    }
}
