//!
//! The `bitwise AND` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `bitwise AND` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseAnd;

impl BitwiseAnd {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseAnd {
    fn into(self) -> Instruction {
        Instruction::BitwiseAnd(self)
    }
}

impl fmt::Display for BitwiseAnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_and")
    }
}
