use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Neg;

impl InstructionInfo for Neg {
    fn to_assembly(&self) -> String {
        "neg".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Neg
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Neg as u8]
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
        Instruction::Neg((*self).clone())
    }
}
