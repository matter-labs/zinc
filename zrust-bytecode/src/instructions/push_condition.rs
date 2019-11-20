use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct PushCondition;

impl Instruction for PushCondition {
    fn to_assembly(&self) -> String {
        "push_cond".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::PushCondition
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::PushCondition as u8]
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl PushCondition {
    pub fn decode(bytes: &[u8]) -> Result<(PushCondition, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::PushCondition, PushCondition)
    }
}
