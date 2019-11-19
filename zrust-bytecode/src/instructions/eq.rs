use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Eq;

impl Instruction for Eq {
    fn to_assembly(&self) -> String {
        "eq".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Eq
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Eq as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Eq {
    pub fn decode(bytes: &[u8]) -> Result<(Eq, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Eq, Eq)
    }
}
