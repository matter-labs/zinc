//!
//! The 'loop end' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopEnd;

impl LoopEnd {
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