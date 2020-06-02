//!
//! The 'no operation' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoOperation;

impl NoOperation {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for NoOperation {
    fn into(self) -> Instruction {
        Instruction::NoOperation(self)
    }
}

impl fmt::Display for NoOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "noop")
    }
}
