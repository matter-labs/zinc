//!
//! The 'contract storage store' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

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

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for StorageStore {
    fn into(self) -> Instruction {
        Instruction::StorageStore(self)
    }
}

impl fmt::Display for StorageStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "storage_store {}", self.size)
    }
}
