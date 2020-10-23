//!
//! The `loop begin` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `loop begin` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopBegin {
    /// The total number of loop iterations.
    pub iterations: usize,
}

impl LoopBegin {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for LoopBegin {
    fn into(self) -> Instruction {
        Instruction::LoopBegin(self)
    }
}

impl fmt::Display for LoopBegin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop_begin {}", self.iterations)
    }
}
