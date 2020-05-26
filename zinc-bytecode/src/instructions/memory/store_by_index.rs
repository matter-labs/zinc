use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Instruction;
use crate::InstructionInfo;

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
}

impl InstructionInfo for StoreByIndex {
    fn to_assembly(&self) -> String {
        format!(
            "store_array_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }

    fn wrap(self) -> Instruction {
        Instruction::StoreByIndex(self)
    }
}
