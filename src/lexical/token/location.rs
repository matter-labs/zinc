//!
//! The token location.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Default, Serialize, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column_start: usize) -> Self {
        Self {
            line,
            column: column_start,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{})", self.line, self.column)
    }
}
