use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Takes `index` from evaluation stack, loads several values from data stack from `address + index` onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadSequenceByIndex {
    pub address: usize,
    pub array_len: usize,
    pub value_len: usize,
}

impl LoadSequenceByIndex {
    pub fn new(address: usize, array_len: usize, value_len: usize) -> Self {
        Self {
            address,
            array_len,
            value_len,
        }
    }
}

impl InstructionInfo for LoadSequenceByIndex {
    fn to_assembly(&self) -> String {
        format!(
            "load_array_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequenceByIndex((*self).clone())
    }
}
