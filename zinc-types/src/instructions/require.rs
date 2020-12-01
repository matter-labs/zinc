//!
//! The `require` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `require` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Require {
    /// The optional error message.
    pub message: Option<String>,
}

impl Require {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(message: Option<String>) -> Self {
        Self { message }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Require {
    fn into(self) -> Instruction {
        Instruction::Require(self)
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            None => write!(f, "require"),
            Some(text) => write!(f, "require \"{}\"", text),
        }
    }
}
