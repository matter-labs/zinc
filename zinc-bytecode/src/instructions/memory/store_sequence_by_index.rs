use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Takes `index` and several values from evaluation stack, stores values in data stack at `address + index`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequenceByIndex {
    pub address: usize,
    pub array_len: usize,
    pub value_len: usize,
}

impl StoreSequenceByIndex {
    pub fn new(address: usize, array_len: usize, value_len: usize) -> Self {
        Self {
            address,
            array_len,
            value_len,
        }
    }
}

impl InstructionInfo for StoreSequenceByIndex {
    fn to_assembly(&self) -> String {
        format!(
            "store_array_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequenceByIndex((*self).clone())
    }
}
