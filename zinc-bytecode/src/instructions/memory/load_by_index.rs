use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Takes `index` from evaluation stack, loads value from data stack from `address + index` onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadByIndex {
    pub address: usize,
    pub len: usize,
}

impl LoadByIndex {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadByIndex {
    fn to_assembly(&self) -> String {
        format!("load_by_index {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadByIndex
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadByIndex((*self).clone())
    }
}
