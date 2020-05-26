use crate::{Instruction, InstructionInfo};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
