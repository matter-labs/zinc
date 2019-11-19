use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Neg;

impl Instruction for Neg {
    fn to_assembly(&self) -> String {
        "neg".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Neg
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Neg as u8]
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Neg {
    pub fn decode(bytes: &[u8]) -> Result<(Neg, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Neg, Neg)
    }
}
