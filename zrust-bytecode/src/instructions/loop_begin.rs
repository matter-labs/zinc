use crate::{Instruction, InstructionCode, DecodingError, utils};
use num_traits::ToPrimitive;
use num_bigint::ToBigInt;

#[derive(Debug)]
pub struct LoopBegin {
    pub iterations: usize
}

impl Instruction for LoopBegin {
    fn to_assembly(&self) -> String {
        format!("loop_begin {}", self.iterations).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::LoopBegin
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::LoopBegin, &self.iterations.to_bigint().unwrap())
    }
}

impl LoopBegin {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn decode(bytes: &[u8]) -> Result<(LoopBegin, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::LoopBegin, bytes)?;
        let iterations = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((LoopBegin { iterations }, len))
    }
}
