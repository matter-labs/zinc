//!
//! The 'store to stack' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Store {
    pub address: usize,
    pub len: usize,
}

impl Store {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }

    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::Store(self)
    }
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "store {} {}", self.address, self.len)
    }
}
