//!
//! The `bitwise NOT` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `bitwise NOT` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseNot;

impl BitwiseNot {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseNot {
    fn into(self) -> Instruction {
        Instruction::BitwiseNot(self)
    }
}

impl fmt::Display for BitwiseNot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_not")
    }
}
