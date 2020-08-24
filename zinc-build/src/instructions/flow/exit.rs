//!
//! The `program exit` instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

///
/// The `program exit` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exit {
    /// The number of fields returned by the program.
    pub output_size: usize,
}

impl Exit {
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

impl Into<Instruction> for Exit {
    fn into(self) -> Instruction {
        Instruction::Exit(self)
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exit {}", self.output_size)
    }
}
