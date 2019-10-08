//!
//! The lexical token location.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{})", self.line, self.column)
    }
}
