use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Add;

impl Instruction for Add {
    fn to_assembly(&self) -> String {
        "add".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Add
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Add as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Add {
    pub fn decode(bytes: &[u8]) -> Result<(Add, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Add, Add)
    }
}
