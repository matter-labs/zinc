//!
//! The `line marker` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `line marker` debug instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineMarker {
    /// The line number starting from `1`.
    pub line: usize,
}

impl LineMarker {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(line: usize) -> Self {
        Self { line }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        true
    }
}

impl Into<Instruction> for LineMarker {
    fn into(self) -> Instruction {
        Instruction::LineMarker(self)
    }
}

impl fmt::Display for LineMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: line = \"{}\"", self.line)
    }
}
