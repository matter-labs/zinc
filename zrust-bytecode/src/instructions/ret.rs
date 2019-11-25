use crate::{utils, DecodingError, InstructionCode, InstructionInfo};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq)]
pub struct Return {
    pub outputs_count: usize,
}

impl Return {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for Return {
    fn to_assembly(&self) -> String {
        "ret".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Return
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Return, &BigInt::from(self.outputs_count))
    }

    fn decode(bytes: &[u8]) -> Result<(Return, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Return, bytes)?;
        let count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(count), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
