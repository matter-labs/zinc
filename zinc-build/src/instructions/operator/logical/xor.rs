//!
//! The `logical XOR` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `logical XOR` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Xor;

impl Xor {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Xor {
    fn into(self) -> Instruction {
        Instruction::Xor(self)
    }
}

impl fmt::Display for Xor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xor")
    }
}
