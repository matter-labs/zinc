//!
//! The `column marker` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `column marker` debug instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnMarker {
    /// The column number starting from `1`.
    pub column: usize,
}

impl ColumnMarker {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(column: usize) -> Self {
        Self { column }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        true
    }
}

impl Into<Instruction> for ColumnMarker {
    fn into(self) -> Instruction {
        Instruction::ColumnMarker(self)
    }
}

impl fmt::Display for ColumnMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: column = \"{}\"", self.column)
    }
}
