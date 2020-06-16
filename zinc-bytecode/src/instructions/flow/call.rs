//!
//! The 'function call' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub address: usize,
    pub inputs_count: usize,
}

impl Call {
    pub fn new(address: usize, inputs_count: usize) -> Self {
        Self {
            address,
            inputs_count,
        }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Call {
    fn into(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call {} {}", self.address, self.inputs_count)
    }
}
