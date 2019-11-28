use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
#[deprecated(note = "dont use frames")]
pub struct FrameBegin;

impl InstructionInfo for FrameBegin {
    fn to_assembly(&self) -> String {
        "frame_begin".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::FrameBegin
    }

    fn encode(&self) -> Vec<u8> {
        vec![Self::code() as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(FrameBegin, usize), DecodingError> {
        utils::decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::FrameBegin((*self).clone())
    }
}
