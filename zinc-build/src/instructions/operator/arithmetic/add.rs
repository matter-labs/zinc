//!
//! The `arithmetic addition` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `arithmetic addition` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Add;

impl Add {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Add {
    fn into(self) -> Instruction {
        Instruction::Add(self)
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "add")
    }
}
