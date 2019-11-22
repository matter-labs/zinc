use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct NoOperation;

impl InstructionInfo for NoOperation {
    fn to_assembly(&self) -> String {
        "noop".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::NoOperation
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::NoOperation as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(NoOperation, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
