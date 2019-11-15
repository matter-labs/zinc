use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct Return;

impl Instruction for Return {
    fn to_assembly(&self) -> String {
        "ret".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Return
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Return as u8]
    }
}

impl Return {
    pub fn decode(bytes: &[u8]) -> Result<(Return, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Return, Return)
    }
}
