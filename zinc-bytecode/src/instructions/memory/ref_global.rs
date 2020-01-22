use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct RefGlobal {
    pub address: usize,
}

impl RefGlobal {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for RefGlobal {
    fn to_assembly(&self) -> String {
        format!("ref_global {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::RefGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.address])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::RefGlobal((*self).clone())
    }
}
