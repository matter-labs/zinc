use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Loads value from storage and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadPush {
    pub index: usize,
}

impl LoadPush {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for LoadPush {
    fn to_assembly(&self) -> String {
        format!("load_push {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadPush
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::LoadPush, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(LoadPush, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::LoadPush, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((LoadPush { index }, len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadPush((*self).clone())
    }
}
