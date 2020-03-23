use crate::{Instruction, InstructionInfo};


use serde_derive::{Deserialize, Serialize};

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreGlobal {
    pub address: usize,
}

impl StoreGlobal {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for StoreGlobal {
    fn to_assembly(&self) -> String {
        format!("store_global {}", self.address)
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreGlobal((*self).clone())
    }
}
