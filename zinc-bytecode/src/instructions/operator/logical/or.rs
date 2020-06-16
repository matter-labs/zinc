//!
//! The 'logical OR' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Or;

impl Or {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Or {
    fn into(self) -> Instruction {
        Instruction::Or(self)
    }
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "or")
    }
}
