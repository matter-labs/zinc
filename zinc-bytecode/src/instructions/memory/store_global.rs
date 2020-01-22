use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreGlobal {
    pub address: usize,
}

impl StoreGlobal {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for StoreGlobal {
    fn to_assembly(&self) -> String {
        format!("store_global {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(
            InstructionCode::StoreGlobal,
            &self.address.to_bigint().unwrap(),
        )
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::StoreGlobal, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((StoreGlobal { address: index }, len))
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
