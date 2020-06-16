//!
//! The 'bitwise shift right' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftRight;

impl BitwiseShiftRight {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseShiftRight {
    fn into(self) -> Instruction {
        Instruction::BitwiseShiftRight(self)
    }
}

impl fmt::Display for BitwiseShiftRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_shift_right")
    }
}
