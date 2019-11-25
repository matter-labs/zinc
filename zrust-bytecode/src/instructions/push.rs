use crate::{utils, DecodingError, InstructionCode, InstructionInfo};
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub struct Push {
    pub value: BigInt,
}

impl Push {
    pub fn new(value: BigInt, _signed: bool, _bit_length: usize) -> Self {
        Push { value }
    }
}

impl InstructionInfo for Push {
    fn to_assembly(&self) -> String {
        format!("push {}", self.value).into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Push
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Push, &self.value)
    }

    fn decode(bytes: &[u8]) -> Result<(Push, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Push, bytes)?;

        Ok((Push { value }, len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
