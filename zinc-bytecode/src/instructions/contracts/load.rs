use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads value from contract's storage onto stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StorageLoad {
    /// Size of value (number of fields)
    pub size: usize,
}

impl StorageLoad {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl InstructionInfo for StorageLoad {
    fn to_assembly(&self) -> String {
        format!("storage_load {}", self.size)
    }

    fn wrap(self) -> Instruction {
        Instruction::StorageLoad(self)
    }
}
