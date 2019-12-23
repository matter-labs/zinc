use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Else;

impl InstructionInfo for Else {
    fn to_assembly(&self) -> String {
        "else".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Else
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Else as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Else((*self).clone())
    }
}
