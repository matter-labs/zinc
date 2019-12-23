use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct Store {
    pub index: usize,
}

impl Store {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for Store {
    fn to_assembly(&self) -> String {
        format!("store {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::Store
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::Store, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::Store, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Store { index }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Store((*self).clone())
    }
}
