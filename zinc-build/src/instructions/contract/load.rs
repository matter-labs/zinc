//!
//! The `contract storage load` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `contract storage load` instruction.
///
/// Loads the value of `size` from the contract storage and loads it onto the evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageLoad {
    /// The size of the loaded value (number of fields).
    pub size: usize,
}

impl StorageLoad {
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
