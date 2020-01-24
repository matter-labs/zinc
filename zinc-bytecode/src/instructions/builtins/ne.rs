use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Ne;

impl InstructionInfo for Ne {
    fn to_assembly(&self) -> String {
        "ne".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Ne
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Ne as u8]
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
        Instruction::Ne((*self).clone())
    }
}
