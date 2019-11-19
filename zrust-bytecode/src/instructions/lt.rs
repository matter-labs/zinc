use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Lt;

impl Instruction for Lt {
    fn to_assembly(&self) -> String {
        "lt".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Lt
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Lt as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Lt {
    pub fn decode(bytes: &[u8]) -> Result<(Lt, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Lt, Lt)
    }
}
