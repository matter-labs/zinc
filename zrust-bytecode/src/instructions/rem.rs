use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Rem;

impl Instruction for Rem {
    fn to_assembly(&self) -> String {
        "rem".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Rem
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Rem as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Rem {
    pub fn decode(bytes: &[u8]) -> Result<(Rem, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Rem, Rem)
    }
}
