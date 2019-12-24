use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct If;

impl InstructionInfo for If {
    fn to_assembly(&self) -> String {
        "if".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::If
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::If as u8]
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

    fn wrap(&self) -> Instruction {
        Instruction::If((*self).clone())
    }
}
