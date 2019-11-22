use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Lt;

impl InstructionInfo for Lt {
    fn to_assembly(&self) -> String {
        "lt".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Lt
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Lt as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Lt, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
