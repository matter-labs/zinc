use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct StoreByRef {
    pub index: usize,
}

impl StoreByRef {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for StoreByRef {
    fn to_assembly(&self) -> String {
        format!("ref_store {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(Self::code(), &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::StoreByRef, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((StoreByRef { index }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreByRef((*self).clone())
    }
}
