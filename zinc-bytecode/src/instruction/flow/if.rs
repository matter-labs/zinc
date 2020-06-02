//!
//! The 'conditional if' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct If;

impl If {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for If {
    fn into(self) -> Instruction {
        Instruction::If(self)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")
    }
}
