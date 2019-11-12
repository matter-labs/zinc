use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct ConditionalSelect;

impl Instruction for ConditionalSelect {
    fn to_assembly(&self) -> String {
        "cs".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::ConditionalSelect
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::ConditionalSelect as u8]
    }
}

impl ConditionalSelect {
    pub fn decode(bytes: &[u8]) -> Result<(ConditionalSelect, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::ConditionalSelect, ConditionalSelect)
    }
}
