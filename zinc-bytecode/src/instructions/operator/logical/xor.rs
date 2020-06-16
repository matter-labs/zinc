//!
//! The 'logical XOR' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Xor;

impl Xor {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Xor {
    fn into(self) -> Instruction {
        Instruction::Xor(self)
    }
}

impl fmt::Display for Xor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xor")
    }
}
