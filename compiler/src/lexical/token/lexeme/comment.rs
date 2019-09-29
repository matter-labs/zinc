//!
//! The comment lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub text: String,
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
