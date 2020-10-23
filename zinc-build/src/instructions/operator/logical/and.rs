//!
//! The `logical AND` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `logical AND` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct And;

impl And {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for And {
    fn into(self) -> Instruction {
        Instruction::And(self)
    }
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "and")
    }
}
