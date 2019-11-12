use crate::{Instruction, InstructionCode, DecodingError};

#[derive(Debug)]
pub struct LoopBegin {
    pub iterations: usize
}

impl Instruction for LoopBegin {
    fn to_assembly(&self) -> String {
        format!("loop_begin {}", self.iterations).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::LoopBegin
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::LoopBegin as u8, 0x01, self.iterations as u8]
    }
}

impl LoopBegin {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn decode(bytes: &[u8]) -> Result<(LoopBegin, usize), DecodingError> {
        if bytes.len() < 3 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::LoopBegin as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            Ok((LoopBegin::new(bytes[2] as usize), 3))
        }
    }
}
