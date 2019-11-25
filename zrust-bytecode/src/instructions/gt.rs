use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Gt;

impl InstructionInfo for Gt {
    fn to_assembly(&self) -> String {
        "gt".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Gt
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Gt as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Gt, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Gt((*self).clone())
    }
}
