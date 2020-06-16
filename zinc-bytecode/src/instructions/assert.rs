//!
//! The 'assert' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assert {
    pub message: Option<String>,
}

impl Assert {
    pub fn new(message: Option<String>) -> Self {
        Self { message }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Assert {
    fn into(self) -> Instruction {
        Instruction::Assert(self)
    }
}

impl fmt::Display for Assert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            None => write!(f, "assert"),
            Some(text) => write!(f, "assert \"{}\"", text),
        }
    }
}
