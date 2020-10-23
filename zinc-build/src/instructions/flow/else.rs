//!
//! The `conditional else` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `conditional else` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Else;

impl Else {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Else {
    fn into(self) -> Instruction {
        Instruction::Else(self)
    }
}

impl fmt::Display for Else {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "else")
    }
}
