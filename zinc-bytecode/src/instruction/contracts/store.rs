//!
//! The 'contract storage store' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

/// Stores value from stack into contract's storage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
