//!
//! The 'store to stack by index' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

/// Takes `index` and several values from evaluation stack, stores values in data stack at `address + index`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoreByIndex {
    pub address: usize,
    pub array_len: usize,
    pub value_len: usize,
}

impl StoreByIndex {
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
        Instruction::StoreByIndex(self)
    }
}

impl fmt::Display for StoreByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "store_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }
}
