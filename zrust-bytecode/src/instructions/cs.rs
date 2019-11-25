use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, InstructionCode, InstructionInfo, Instruction};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ConditionalSelect;

impl InstructionInfo for ConditionalSelect {
    fn to_assembly(&self) -> String {
        "cs".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::ConditionalSelect
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::ConditionalSelect as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(ConditionalSelect, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        3
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::ConditionalSelect((*self).clone())
    }
}
