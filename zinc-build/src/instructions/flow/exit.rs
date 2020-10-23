//!
//! The `exit` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `exit` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exit {
    /// The number of fields returned by the application.
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
