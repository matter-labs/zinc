use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Mul;

impl Instruction for Mul {
    fn to_assembly(&self) -> String {
        "mul".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Mul
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Mul as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Mul {
    pub fn decode(bytes: &[u8]) -> Result<(Mul, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Mul, Mul)
    }
}
