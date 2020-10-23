//!
//! The `contract storage store` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `contract storage store` instruction.
///
/// Stores the value of `size` from the evaluation stack in the contract storage.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageStore {
    /// The size of the stored value (number of fields).
    pub size: usize,
}

impl StorageStore {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
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
