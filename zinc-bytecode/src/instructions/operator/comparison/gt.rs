//!
//! The 'greater comparison' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gt;

impl Gt {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Gt {
    fn into(self) -> Instruction {
        Instruction::Gt(self)
    }
}

impl fmt::Display for Gt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gt")
    }
}