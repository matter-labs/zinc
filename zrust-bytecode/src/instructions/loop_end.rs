use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct LoopEnd;

impl InstructionInfo for LoopEnd {
    fn to_assembly(&self) -> String {
        "loop_end".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::LoopEnd
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::LoopEnd as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(LoopEnd, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
