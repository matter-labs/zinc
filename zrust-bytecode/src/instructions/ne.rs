use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default)]
pub struct Ne;

impl InstructionInfo for Ne {
    fn to_assembly(&self) -> String {
        "ne".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Ne
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Ne as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Ne, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
