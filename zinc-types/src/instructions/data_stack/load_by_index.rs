//!
//! The `load from stack by index` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `load from stack by index` instruction.
///
/// Takes `index` from evaluation stack, loads several values from the data stack
/// at `address + index` onto the evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadByIndex {
    /// The start address where the data must be loaded from.
    pub address: usize,
    /// The size of the data chunk which must be loaded.
    pub value_size: usize,
    /// The total size of the data chunk where the linear scan must be applied.
    pub total_size: usize,
}

impl LoadByIndex {
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

impl Into<Instruction> for LoadByIndex {
    fn into(self) -> Instruction {
        Instruction::LoadByIndex(self)
    }
}

impl fmt::Display for LoadByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "load_by_index {} {} {}",
            self.address, self.value_size, self.total_size
        )
    }
}
