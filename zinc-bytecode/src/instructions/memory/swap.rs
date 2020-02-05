use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Swap;

impl InstructionInfo for Swap {
    fn to_assembly(&self) -> String {
        "swap".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Swap
    }

    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn decode(_bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        unimplemented!()
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        2
    }

    fn wrap(&self) -> Instruction {
        Instruction::Swap((*self).clone())
    }
}
