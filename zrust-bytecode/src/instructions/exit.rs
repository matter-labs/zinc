use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Exit;

impl Instruction for Exit {
    fn to_assembly(&self) -> String {
        "exit".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Exit
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Exit as u8]
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}

impl Exit {
    pub fn decode(bytes: &[u8]) -> Result<(Exit, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Exit, Exit)
    }
}
