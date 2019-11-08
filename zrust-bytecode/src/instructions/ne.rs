use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Ne;

impl Instruction for Ne {
    fn to_assembly(&self) -> String {
        "ne".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Ne
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Ne as u8]
    }
}

impl Ne {
    pub fn decode(bytes: &[u8]) -> Result<(Ne, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Ne, Ne)
    }
}
