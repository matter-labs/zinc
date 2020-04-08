use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadSequence {
    pub address: usize,
    pub len: usize,
}

impl LoadSequence {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadSequence {
    fn to_assembly(&self) -> String {
        format!("load_array {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequence((*self).clone())
    }
}
