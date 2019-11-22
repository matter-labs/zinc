use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Exit;

impl InstructionInfo for Exit {
    fn to_assembly(&self) -> String {
        "exit".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Exit
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Exit as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Exit, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
