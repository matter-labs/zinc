use crate::{Instruction, InstructionCode, DecodingError};

#[derive(Debug)]
pub struct Copy {
    value: usize
}

impl Instruction for Copy {
    fn to_assembly(&self) -> String {
        format!("copy {}", self.value).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Copy
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Copy as u8, 0x01, self.value as u8]
    }
}

impl Copy {
    pub fn decode(bytes: &[u8]) -> Result<(Copy, usize), DecodingError> {
        if bytes.len() < 3 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::Copy as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            Ok((Copy { value: bytes[2] as usize }, 3))
        }
    }
}
