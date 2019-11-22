use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Not;

impl InstructionInfo for Not {
    fn to_assembly(&self) -> String {
        "not".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Not
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Not as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Not, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
