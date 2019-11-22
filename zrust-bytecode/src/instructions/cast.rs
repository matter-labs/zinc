use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Cast;

impl Cast {
    pub fn new(_signed: bool, _length: u8) -> Self{
        Cast
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        "cast".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Cast
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Cast as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Cast, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
