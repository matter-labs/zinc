//!
//! The 'load from stack by index' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

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
}

impl InstructionInfo for LoadByIndex {
    fn to_assembly(&self) -> String {
        format!(
            "load_array_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }

    fn wrap(self) -> Instruction {
        Instruction::LoadByIndex(self)
    }
}
