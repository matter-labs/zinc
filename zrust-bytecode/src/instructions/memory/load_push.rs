use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Loads value from storage and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadPush {
    pub index: usize,
}

impl LoadPush {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for LoadPush {
    fn to_assembly(&self) -> String {
        format!("load_push {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadPush
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.index])
    }

    fn decode(bytes: &[u8]) -> Result<(LoadPush, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadPush((*self).clone())
    }
}

impl From<usize> for LoadPush {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
