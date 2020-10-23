//!
//! The `evaluation stack slice` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `evaluation stack slice` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Slice {
    /// The size of the sliced chunk.
    pub slice_length: usize,
    /// The total size of the data chunk to be sliced.
    pub total_size: usize,
}

impl Slice {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(slice_length: usize, total_size: usize) -> Self {
        Self {
            slice_length,
            total_size,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Slice {
    fn into(self) -> Instruction {
        Instruction::Slice(self)
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slice {} {}", self.slice_length, self.total_size)
    }
}
