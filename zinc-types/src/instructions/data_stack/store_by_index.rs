//!
//! The `store to stack by index` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `store to stack by index` instruction.
///
/// Takes `index` and several values from evaluation stack,
/// stores the values in the data stack at `address + index`.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoreByIndex {
    /// The start address where the data must be stored to.
    pub address: usize,
    /// The size of the data chunk which must be stored.
    pub value_size: usize,
    /// The total size of the data chunk where the linear scan must be applied.
    pub total_size: usize,
}

impl StoreByIndex {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: usize, value_size: usize, total_size: usize) -> Self {
        Self {
            address,
            value_size,
            total_size,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for StoreByIndex {
    fn into(self) -> Instruction {
        Instruction::StoreByIndex(self)
    }
}

impl fmt::Display for StoreByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "store_by_index {} {} {}",
            self.address, self.value_size, self.total_size
        )
    }
}
