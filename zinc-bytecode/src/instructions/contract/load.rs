//!
//! The 'contract storage load' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

/// Loads value from contract's storage onto stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageLoad {
    /// Size of value (number of fields)
    pub size: usize,
}

impl StorageLoad {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for StorageLoad {
    fn into(self) -> Instruction {
        Instruction::StorageLoad(self)
    }
}

impl fmt::Display for StorageLoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "storage_load {}", self.size)
    }
}
