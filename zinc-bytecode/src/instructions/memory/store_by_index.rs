use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Takes `index` and value from evaluation stack, stores value in data stack at `address + index`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreByIndex {
    pub address: usize,
    pub len: usize,
}

impl StoreByIndex {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreByIndex {
    fn to_assembly(&self) -> String {
        format!("store_by_index {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreByIndex((*self).clone())
    }
}
