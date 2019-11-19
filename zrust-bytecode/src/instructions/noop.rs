use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug)]
pub struct NoOperation;

impl Instruction for NoOperation {
    fn to_assembly(&self) -> String {
        "noop".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::NoOperation
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::NoOperation as u8]
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}

impl NoOperation {
    pub fn decode(bytes: &[u8]) -> Result<(NoOperation, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::NoOperation, NoOperation)
    }
}
