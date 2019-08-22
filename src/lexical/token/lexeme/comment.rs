//!
//! The comment lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Comment {
    text: String,
}

impl Comment {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
