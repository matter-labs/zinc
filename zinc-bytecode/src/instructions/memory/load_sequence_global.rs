use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadSequenceGlobal {
    pub address: usize,
    pub len: usize,
}

impl LoadSequenceGlobal {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadSequenceGlobal {
    fn to_assembly(&self) -> String {
        format!("load_array_global {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequenceGlobal((*self).clone())
    }
}
