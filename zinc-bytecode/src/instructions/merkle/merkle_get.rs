use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Loads value from storage and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct MerkleGet {
    pub address: usize,
    pub size: usize,
}

impl MerkleGet {
    pub fn new(address: usize, size: usize) -> Self {
        Self { address, size }
    }
}

impl InstructionInfo for MerkleGet {
    fn to_assembly(&self) -> String {
        format!("merkle_get {} {}", self.address, self.size)
    }

    fn code() -> InstructionCode {
        InstructionCode::MerkleGet
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.size])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        self.size
    }

    fn wrap(&self) -> Instruction {
        Instruction::MerkleGet((*self).clone())
    }
}
