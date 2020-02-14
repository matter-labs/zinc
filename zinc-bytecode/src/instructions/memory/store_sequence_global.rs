use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequenceGlobal {
    pub address: usize,
    pub len: usize,
}

impl StoreSequenceGlobal {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreSequenceGlobal {
    fn to_assembly(&self) -> String {
        format!("store_sequence_global {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreSequenceGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        self.len
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequenceGlobal((*self).clone())
    }
}