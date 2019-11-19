use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Or;

impl Instruction for Or {
    fn to_assembly(&self) -> String {
        "or".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Or
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Or as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Or {
    pub fn decode(bytes: &[u8]) -> Result<(Or, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Or, Or)
    }
}
