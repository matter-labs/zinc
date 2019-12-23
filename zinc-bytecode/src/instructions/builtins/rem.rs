use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Rem;

impl InstructionInfo for Rem {
    fn to_assembly(&self) -> String {
        "rem".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Rem
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Rem as u8]
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
        Instruction::Rem((*self).clone())
    }
}
