use crate::{Instruction, InstructionCode, DecodingError, utils};
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
        utils::encode_with_vlq_argument(InstructionCode::Push, &self.value)
    }
}

impl Push {
    pub fn new(value: BigInt, _signed: bool, _bit_length: usize) -> Self {
        Push { value }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Push, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Push, bytes)?;

        Ok((Push { value }, len))
    }
}
