//!
//! The `conditional if end` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `conditional if end` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndIf;

impl EndIf {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for EndIf {
    fn into(self) -> Instruction {
        Instruction::EndIf(self)
    }
}

impl fmt::Display for EndIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "endif")
    }
}
