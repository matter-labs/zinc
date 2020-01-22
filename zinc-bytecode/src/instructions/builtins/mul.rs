use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Mul;

impl InstructionInfo for Mul {
    fn to_assembly(&self) -> String {
        "mul".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Mul
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Mul as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Mul((*self).clone())
    }
}
