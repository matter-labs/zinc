//!
//! The 'load from stack' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Load {
    pub address: usize,
    pub len: usize,
}

impl Load {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Load {
    fn into(self) -> Instruction {
        Instruction::Load(self)
    }
}

impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "load {} {}", self.address, self.len,)
    }
}
