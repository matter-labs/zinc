//!
//! The lexical token comment lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Comment {
    Line { inner: String },
    Block { inner: String },
}

impl Comment {
    pub fn new_line(inner: String) -> Self {
        Self::Line { inner }
    }

    pub fn new_block(inner: String) -> Self {
        Self::Block { inner }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Line { inner } => write!(f, "{}", inner),
            Self::Block { inner } => write!(f, "{}", inner),
        }
    }
}
