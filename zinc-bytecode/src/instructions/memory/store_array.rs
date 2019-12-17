use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo, utils};

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct StoreArray {
    pub address: usize,
    pub len: usize,
}

impl StoreArray {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreArray {
    fn to_assembly(&self) -> String {
        format!("store_array {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreArray
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(StoreArray, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((
            Self::new(args[0], args[1]),
            len,
        ))
    }

    fn inputs_count(&self) -> usize {
        self.len
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreArray((*self).clone())
    }
}
