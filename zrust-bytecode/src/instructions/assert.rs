use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Assert;

impl Instruction for Assert {
    fn to_assembly(&self) -> String {
        "assert".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Assert
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Assert as u8]
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }
}

impl Assert {
    pub fn decode(bytes: &[u8]) -> Result<(Assert, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Assert, Assert)
    }
}
