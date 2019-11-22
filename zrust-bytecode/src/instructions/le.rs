use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Le;

impl InstructionInfo for Le {
    fn to_assembly(&self) -> String {
        "le".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Le
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Le as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Le, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

