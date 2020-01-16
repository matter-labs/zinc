use crate::instructions::utils;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct Cast {
    pub signed: bool,
    pub length: usize,
}

impl Cast {
    pub fn new(signed: bool, length: usize) -> Self {
        Self { signed, length }
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        format!("cast {} {}", self.signed, self.length)
    }

    fn code() -> InstructionCode {
        InstructionCode::Cast
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.signed as usize, self.length])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;
        let signed = match args[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodingError::ConstantTooLong),
        }?;
        Ok((Self::new(signed, args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Cast((*self).clone())
    }
}
