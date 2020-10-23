//!
//! The `bitwise OR` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `bitwise OR` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseOr;

impl BitwiseOr {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseOr {
    fn into(self) -> Instruction {
        Instruction::BitwiseOr(self)
    }
}

impl fmt::Display for BitwiseOr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_or")
    }
}
