//!
//! The 'bitwise XOR' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseXor;

impl BitwiseXor {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for BitwiseXor {
    fn into(self) -> Instruction {
        Instruction::BitwiseXor(self)
    }
}

impl fmt::Display for BitwiseXor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_xor")
    }
}
