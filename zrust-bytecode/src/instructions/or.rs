use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Or;

impl InstructionInfo for Or {
    fn to_assembly(&self) -> String {
        "or".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Or
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Or as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Or, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
