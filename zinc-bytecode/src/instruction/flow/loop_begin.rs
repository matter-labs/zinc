//!
//! The 'loop begin' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopBegin {
    pub iterations: usize,
}

impl LoopBegin {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

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
