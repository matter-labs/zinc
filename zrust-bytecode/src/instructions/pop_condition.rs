use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct PopCondition;

impl InstructionInfo for PopCondition {
    fn to_assembly(&self) -> String {
        "pop_cond".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::PopCondition
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::PopCondition as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(PopCondition, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::PopCondition((*self).clone())
    }
}
