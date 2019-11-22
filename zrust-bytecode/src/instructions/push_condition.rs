use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct PushCondition;

impl InstructionInfo for PushCondition {
    fn to_assembly(&self) -> String {
        "push_cond".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::PushCondition
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::PushCondition as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(PushCondition, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
