use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Loads value from storage and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct MerkleSet {
    pub address: usize,
    pub size: usize,
}

impl MerkleSet {
    pub fn new(address: usize, size: usize) -> Self {
        Self { address, size }
    }
}

impl InstructionInfo for MerkleSet {
    fn to_assembly(&self) -> String {
        format!("merkle_set {} {}", self.address, self.size)
    }

    fn code() -> InstructionCode {
        InstructionCode::MerkleSet
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address, self.size])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1 + self.size
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::MerkleSet((*self).clone())
    }
}
