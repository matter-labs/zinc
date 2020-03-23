use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequence {
    pub address: usize,
    pub len: usize,
}

impl StoreSequence {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreSequence {
    fn to_assembly(&self) -> String {
        format!("store_array {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequence((*self).clone())
    }
}
