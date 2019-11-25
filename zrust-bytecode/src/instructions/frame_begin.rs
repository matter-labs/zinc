use crate::{DecodingError, InstructionCode, InstructionInfo, utils};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq)]
pub struct FrameBegin {
    pub inputs_count: usize,
}

impl FrameBegin {
    pub fn new(inputs_count: usize) -> Self {
        Self { inputs_count }
    }
}

impl InstructionInfo for FrameBegin {
    fn to_assembly(&self) -> String {
        format!("frame_begin {}", self.inputs_count).into()
    }

    fn code() -> InstructionCode {
        InstructionCode::FrameBegin
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(Self::code(), &BigInt::from(self.inputs_count))
    }

    fn decode(bytes: &[u8]) -> Result<(FrameBegin, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(Self::code(), bytes)?;
        let inputs_count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(inputs_count), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}
