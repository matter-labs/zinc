//!
//! The 'bitwise shift left' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftLeft;

impl BitwiseShiftLeft {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseShiftLeft {
    fn into(self) -> Instruction {
        Instruction::BitwiseShiftLeft(self)
    }
}

impl fmt::Display for BitwiseShiftLeft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_shift_left")
    }
}
