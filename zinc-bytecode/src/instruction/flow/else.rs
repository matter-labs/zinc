//!
//! The 'conditional else' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Else;

impl Else {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Else {
    fn into(self) -> Instruction {
        Instruction::Else(self)
    }
}

impl fmt::Display for Else {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "else")
    }
}
