use crate::{Instruction, InstructionCode, DecodingError};

#[derive(Debug)]
pub struct Pop {
    pub count: usize
}

impl Instruction for Pop {
    fn to_assembly(&self) -> String {
        format!("pop {}", self.count).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Pop
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Pop as u8, self.count as u8]
    }
}

impl Pop {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Pop, usize), DecodingError> {
        if bytes.len() < 2 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::Pop as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            Ok((Pop::new(bytes[1] as usize), 2))
        }
    }
}
