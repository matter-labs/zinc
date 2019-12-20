use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo, utils};

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadSequence {
    pub address: usize,
    pub len: usize,
}

impl LoadSequence {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadSequence {
    fn to_assembly(&self) -> String {
        format!("load_array {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadSequence
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(LoadSequence, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((
            Self::new(args[0], args[1]),
            len,
        ))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        self.len
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequence((*self).clone())
    }
}
