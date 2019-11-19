use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Cast;

impl Instruction for Cast {
    fn to_assembly(&self) -> String {
        "cast".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Cast
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Cast as u8]
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Cast {
    pub fn new(_signed: bool, _length: u8) -> Self{
        Cast
    }

    pub fn decode(bytes: &[u8]) -> Result<(Cast, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Cast, Cast)
    }
}
