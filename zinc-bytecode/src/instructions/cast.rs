use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Cast {
    signed: bool,
    length: usize,
}

impl Cast {
    pub fn new(signed: bool, length: u8) -> Self {
        Self {
            signed,
            length: length as usize,
        }
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        format!("cast {} {}", self.signed, self.length)
    }

    fn code() -> InstructionCode {
        InstructionCode::Cast
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Cast as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Cast((*self).clone())
    }
}
