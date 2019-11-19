use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Ge;

impl Instruction for Ge {
    fn to_assembly(&self) -> String {
        "ge".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Ge
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Ge as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Ge {
    pub fn decode(bytes: &[u8]) -> Result<(Ge, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Ge, Ge)
    }
}
