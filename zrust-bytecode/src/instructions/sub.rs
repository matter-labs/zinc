use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Sub;

impl Instruction for Sub {
    fn to_assembly(&self) -> String {
        "sub".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Sub
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Sub as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Sub {
    pub fn decode(bytes: &[u8]) -> Result<(Sub, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Sub, Sub)
    }
}
