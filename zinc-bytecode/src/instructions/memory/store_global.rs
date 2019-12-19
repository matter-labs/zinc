use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct StoreGlobal {
    pub index: usize,
}

impl StoreGlobal {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for StoreGlobal {
    fn to_assembly(&self) -> String {
        format!("store_global {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::StoreGlobal, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(StoreGlobal, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::StoreGlobal, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((StoreGlobal { index }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreGlobal((*self).clone())
    }
}
