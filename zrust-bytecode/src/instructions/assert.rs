use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Assert;

impl InstructionInfo for Assert {
    fn to_assembly(&self) -> String {
        "assert".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Assert
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Assert as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
