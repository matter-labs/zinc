use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Le;

impl Instruction for Le {
    fn to_assembly(&self) -> String {
        "le".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Le
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Le as u8]
    }
}

impl Le {
    pub fn decode(bytes: &[u8]) -> Result<(Le, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Le, Le)
    }
}
