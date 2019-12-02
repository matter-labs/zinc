use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Removes value from the top of the stack and stores it in the storage.
#[derive(Debug, PartialEq, Clone)]
pub struct PopStoreArray {
    pub address: usize,
    pub len: usize,
}

impl PopStoreArray {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for PopStoreArray {
    fn to_assembly(&self) -> String {
        format!("pop_store_array {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::PopStoreArray
    }

    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn decode(_bytes: &[u8]) -> Result<(PopStoreArray, usize), DecodingError> {
        unimplemented!()
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::PopStoreArray((*self).clone())
    }
}
