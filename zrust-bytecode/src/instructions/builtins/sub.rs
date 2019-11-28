use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Sub;

impl InstructionInfo for Sub {
    fn to_assembly(&self) -> String {
        "sub".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Sub
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Sub as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Sub, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Sub((*self).clone())
    }
}
