//!
//! The 'load from stack by index' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

/// Takes `index` from evaluation stack, loads several values from data stack from `address + index` onto evaluation stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadByIndex {
    pub address: usize,
    pub array_len: usize,
    pub value_len: usize,
}

impl LoadByIndex {
    pub fn new(address: usize, array_len: usize, value_len: usize) -> Self {
        Self {
            address,
            array_len,
            value_len,
        }
    }

    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::LoadByIndex(self)
    }
}

impl fmt::Display for LoadByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "load_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }
}
