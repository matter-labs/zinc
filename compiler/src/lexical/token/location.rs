//!
//! The token location.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Default, Serialize, Clone, Copy, PartialEq)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{})", self.line, self.column)
    }
}
