use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Pop;

impl Instruction for Pop {
    fn to_assembly(&self) -> String {
        "pop".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Pop
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Pop as u8]
    }
}

impl Pop {
    pub fn decode(bytes: &[u8]) -> Result<(Pop, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Pop, Pop)
    }
}
