use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequenceGlobal {
    pub address: usize,
    pub len: usize,
}

impl StoreSequenceGlobal {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreSequenceGlobal {
    fn to_assembly(&self) -> String {
        format!("store_sequence_global {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequenceGlobal((*self).clone())
    }
}
