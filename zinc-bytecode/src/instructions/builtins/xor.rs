use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Xor;

impl InstructionInfo for Xor {
    fn to_assembly(&self) -> String {
        "xor".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Xor
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Xor as u8]
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
        Instruction::Xor((*self).clone())
    }
}
