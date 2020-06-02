//!
//! The 'arithmetic subtraction' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sub;

impl Sub {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Sub {
    fn into(self) -> Instruction {
        Instruction::Sub(self)
    }
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sub")
    }
}
