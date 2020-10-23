//!
//! The `bitwise shift left` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `bitwise shift left` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftLeft;

impl BitwiseShiftLeft {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseShiftLeft {
    fn into(self) -> Instruction {
        Instruction::BitwiseShiftLeft(self)
    }
}

impl fmt::Display for BitwiseShiftLeft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_shift_left")
    }
}
