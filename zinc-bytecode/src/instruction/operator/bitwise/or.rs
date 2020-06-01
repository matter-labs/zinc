//!
//! The 'bitwise OR' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseOr;

impl BitwiseOr {
    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::BitwiseOr(self)
    }
}

impl fmt::Display for BitwiseOr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitwise_or")
    }
}
