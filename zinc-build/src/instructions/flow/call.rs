//!
//! The `function call` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `function call` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Call {
    /// The function start index in the bytecode.
    pub address: usize,
    /// The function arguments size in field elements.
    pub input_size: usize,
}

impl Call {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: usize, input_size: usize) -> Self {
        Self {
            address,
            input_size,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Call {
    fn into(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call {} {}", self.address, self.input_size)
    }
}
