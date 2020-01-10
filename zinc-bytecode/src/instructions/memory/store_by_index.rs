use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Takes `index` and value from evaluation stack, stores value in data stack at `address + index`.
#[derive(Debug, PartialEq, Clone)]
pub struct StoreByIndex {
    pub address: usize,
    pub len: usize,
}

impl StoreByIndex {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreByIndex {
    fn to_assembly(&self) -> String {
        format!("store_by_index {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreByIndex
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreByIndex((*self).clone())
    }
}
