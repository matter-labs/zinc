//!
//! The 'line marker' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineMarker {
    pub line: usize,
}

impl LineMarker {
    pub fn new(line: usize) -> Self {
        Self { line }
    }

    pub fn is_debug(&self) -> bool {
        true
    }

    pub fn wrap(self) -> Instruction {
        Instruction::LineMarker(self)
    }
}

impl fmt::Display for LineMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: line = \"{}\"", self.line)
    }
}
