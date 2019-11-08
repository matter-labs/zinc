use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Gt;

impl Instruction for Gt {
    fn to_assembly(&self) -> String {
        "gt".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Gt
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Gt as u8]
    }
}

impl Gt {
    pub fn decode(bytes: &[u8]) -> Result<(Gt, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Gt, Gt)
    }
}
