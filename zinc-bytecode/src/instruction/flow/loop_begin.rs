//!
//! The 'loop begin' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopBegin {
    pub iterations: usize,
}

impl LoopBegin {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl InstructionInfo for LoopBegin {
    fn to_assembly(&self) -> String {
        format!("loop_begin {}", self.iterations)
    }

    fn wrap(self) -> Instruction {
        Instruction::LoopBegin(self)
    }
}
