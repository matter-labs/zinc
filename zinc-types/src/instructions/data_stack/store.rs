//!
//! The `store to stack` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `store to stack` instruction.
///
/// Stores several values from the evaluation stack in the data stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Store {
    /// The start address where the data must be stored to.
    pub address: usize,
    /// The size of data that must be stored from the bottom to top.
    pub size: usize,
}

impl Store {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: usize, size: usize) -> Self {
        Self { address, size }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Store {
    fn into(self) -> Instruction {
        Instruction::Store(self)
    }
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "store {} {}", self.address, self.size)
    }
}
