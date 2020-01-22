use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Div;

impl InstructionInfo for Div {
    fn to_assembly(&self) -> String {
        "div".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Div
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Div as u8]
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
        Instruction::Div((*self).clone())
    }
}
