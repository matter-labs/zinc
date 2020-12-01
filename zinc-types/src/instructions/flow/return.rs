//!
//! The `function return` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `function return` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Return {
    /// The number of fields returned and put onto the evaluation stack.
    pub output_size: usize,
}

impl Return {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(output_size: usize) -> Self {
        Self { output_size }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Return {
    fn into(self) -> Instruction {
        Instruction::Return(self)
    }
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.output_size)
    }
}
