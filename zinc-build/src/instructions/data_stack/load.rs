//!
//! The `load from stack` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `load from stack` instruction.
///
/// Loads several values from the data stack and pushes them onto the evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Load {
    /// The start address where the data must be loaded from.
    pub address: usize,
    /// The size of data that must be loaded from the bottom to top.
    pub size: usize,
}

impl Load {
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

impl Into<Instruction> for Load {
    fn into(self) -> Instruction {
        Instruction::Load(self)
    }
}

impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "load {} {}", self.address, self.size,)
    }
}
