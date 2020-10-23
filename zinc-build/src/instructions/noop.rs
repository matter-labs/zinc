//!
//! The `no operation` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `no operation` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoOperation;

impl NoOperation {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for NoOperation {
    fn into(self) -> Instruction {
        Instruction::NoOperation(self)
    }
}

impl fmt::Display for NoOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "noop")
    }
}
