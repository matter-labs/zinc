use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo, utils};

/// Removes value from the top of the stack and stores it in the storage.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadPushByIndex {
    pub address: usize,
    pub len: usize,
}

impl LoadPushByIndex {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadPushByIndex {
    fn to_assembly(&self) -> String {
        format!("load_push_by_index {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadPushByIndex
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(LoadPushByIndex, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((
            Self::new(args[0], args[1]),
            len,
        ))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadPushByIndex((*self).clone())
    }
}
