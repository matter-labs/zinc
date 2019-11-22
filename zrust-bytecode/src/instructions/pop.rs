use crate::{InstructionInfo, InstructionCode, DecodingError, utils};
use num_traits::ToPrimitive;
use num_bigint::ToBigInt;

#[derive(Debug,PartialEq,Default)]
pub struct Pop {
    pub count: usize
}

impl Pop {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl InstructionInfo for Pop {
    fn to_assembly(&self) -> String {
        format!("pop {}", self.count).into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Pop
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Pop, &self.count.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(Pop, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Pop, bytes)?;
        let count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Pop { count }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
