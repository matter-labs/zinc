use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default)]
pub struct Add;

impl InstructionInfo for Add {
    fn to_assembly(&self) -> String {
        "add".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Add
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Add as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
