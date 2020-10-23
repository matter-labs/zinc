//!
//! The `bitwise shift right` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `bitwise shift right` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftRight;

impl BitwiseShiftRight {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseShiftRight {
    fn into(self) -> Instruction {
        Instruction::BitwiseShiftRight(self)
    }
}

impl fmt::Display for BitwiseShiftRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_shift_right")
    }
}
