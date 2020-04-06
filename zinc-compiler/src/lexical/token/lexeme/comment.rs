//!
//! The lexical token comment lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub inner: String,
}

impl Comment {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
