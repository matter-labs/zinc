//!
//! The 'greater or equal comparison' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ge;

impl Ge {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Ge {
    fn into(self) -> Instruction {
        Instruction::Ge(self)
    }
}

impl fmt::Display for Ge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ge")
    }
}
