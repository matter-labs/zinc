use crate::{DecodingError, InstructionCode, InstructionInfo, utils};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq)]
pub struct FrameEnd {
    pub outputs_count: usize,
}

impl FrameEnd {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for FrameEnd {
    fn to_assembly(&self) -> String {
        format!("frame_end {}", self.outputs_count).into()
    }

    fn code() -> InstructionCode {
        InstructionCode::FrameEnd
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(Self::code(), &BigInt::from(self.outputs_count))
    }

    fn decode(bytes: &[u8]) -> Result<(FrameEnd, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(Self::code(), bytes)?;
        let outputs_count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(outputs_count), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
