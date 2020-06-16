//!
//! The 'lesser comparison' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lt;

impl Lt {
    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::Lt(self)
    }
}

impl Into<Instruction> for Lt {
    fn into(self) -> Instruction {
        Instruction::Lt(self)
    }
}

impl fmt::Display for Lt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lt")
    }
}
