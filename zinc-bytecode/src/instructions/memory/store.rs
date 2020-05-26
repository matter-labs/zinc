use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Instruction;
use crate::InstructionInfo;

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Store {
    pub address: usize,
    pub len: usize,
}

impl Store {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for Store {
    fn to_assembly(&self) -> String {
        format!("store_array {} {}", self.address, self.len)
    }

    fn wrap(self) -> Instruction {
        Instruction::Store(self)
    }
}
