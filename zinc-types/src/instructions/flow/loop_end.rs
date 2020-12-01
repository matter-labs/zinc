//!
//! The `loop end` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `loop end` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopEnd;

impl LoopEnd {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for LoopEnd {
    fn into(self) -> Instruction {
        Instruction::LoopEnd(self)
    }
}

impl fmt::Display for LoopEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop_end")
    }
}
