use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Eq;

impl InstructionInfo for Eq {
    fn to_assembly(&self) -> String {
        "eq".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Eq
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Eq as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Eq, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
