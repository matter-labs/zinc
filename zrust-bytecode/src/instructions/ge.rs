use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Ge;

impl InstructionInfo for Ge {
    fn to_assembly(&self) -> String {
        "ge".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Ge
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Ge as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Ge, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Ge((*self).clone())
    }
}
