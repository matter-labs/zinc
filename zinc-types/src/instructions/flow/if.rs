//!
//! The `conditional if` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `conditional if` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct If;

impl If {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for If {
    fn into(self) -> Instruction {
        Instruction::If(self)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")
    }
}
