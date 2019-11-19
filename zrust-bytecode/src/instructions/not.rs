use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Not;

impl Instruction for Not {
    fn to_assembly(&self) -> String {
        "not".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Not
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Not as u8]
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Not {
    pub fn decode(bytes: &[u8]) -> Result<(Not, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Not, Not)
    }
}
