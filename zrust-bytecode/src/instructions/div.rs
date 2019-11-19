use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Div;

impl Instruction for Div {
    fn to_assembly(&self) -> String {
        "div".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Div
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Div as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Div {
    pub fn decode(bytes: &[u8]) -> Result<(Div, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Div, Div)
    }
}
