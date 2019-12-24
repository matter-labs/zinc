use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Loads value from data stack and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadGlobal {
    pub address: usize,
}

impl LoadGlobal {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for LoadGlobal {
    fn to_assembly(&self) -> String {
        format!("load_global {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
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
        Instruction::LoadGlobal((*self).clone())
    }
}
