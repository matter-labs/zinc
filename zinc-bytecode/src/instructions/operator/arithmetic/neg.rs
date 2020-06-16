//!
//! The 'arithmetic negation' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Neg;

impl Neg {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Neg {
    fn into(self) -> Instruction {
        Instruction::Neg(self)
    }
}

impl fmt::Display for Neg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "neg")
    }
}
