use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Loads value from storage and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct MerkleInit {
    pub address: usize,
}

impl MerkleInit {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for MerkleInit {
    fn to_assembly(&self) -> String {
        format!("merkle_init {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::MerkleInit
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::MerkleInit((*self).clone())
    }
}
