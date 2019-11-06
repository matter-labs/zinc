use crate::{Instruction, InstructionCode, DecodingError};
use num_bigint::BigInt;

#[derive(Debug)]
pub struct Push {
    pub value: BigInt
}

impl Instruction for Push {
    fn to_assembly(&self) -> String {
        format!("push {}", self.value).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Push
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Push as u8, 0x01, self.value.to_bytes_le().1[0]]
    }
}

impl Push {
    pub fn decode(bytes: &[u8]) -> Result<(Push, usize), DecodingError> {
        if bytes.len() < 3 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::Push as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            Ok((Push { value: BigInt::from(bytes[2]) }, 3))
        }
    }
}
