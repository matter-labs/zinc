use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Stores value from stack into contract's storage.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StorageStore {
    /// Size of value (number of fields)
    pub size: usize,
}

impl StorageStore {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl InstructionInfo for StorageStore {
    fn to_assembly(&self) -> String {
        format!("storage_store {}", self.size)
    }

    fn wrap(self) -> Instruction {
        Instruction::StorageStore(self)
    }
}
