//!
//! The `evaluation stack copy` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `evaluation stack copy` instruction.
///
/// Copies the top element of the evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Copy;

impl Copy {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Copy {
    fn into(self) -> Instruction {
        Instruction::Copy(self)
    }
}

impl fmt::Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "copy",)
    }
}
