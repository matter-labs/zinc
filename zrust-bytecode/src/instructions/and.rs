use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct And;

impl Instruction for And {
    fn to_assembly(&self) -> String {
        "and".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::And
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::And as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl And {
    pub fn decode(bytes: &[u8]) -> Result<(And, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::And, And)
    }
}
