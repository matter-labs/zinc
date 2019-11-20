use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct PopCondition;

impl Instruction for PopCondition {
    fn to_assembly(&self) -> String {
        "pop_cond".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::PopCondition
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::PopCondition as u8]
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}

impl PopCondition {
    pub fn decode(bytes: &[u8]) -> Result<(PopCondition, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::PopCondition, PopCondition)
    }
}
