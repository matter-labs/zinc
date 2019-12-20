use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct RefStore {
    pub index: usize,
}

impl RefStore {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for RefStore {
    fn to_assembly(&self) -> String {
        format!("ref_store")
    }

    fn code() -> InstructionCode {
        InstructionCode::RefStore
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::RefStore, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(RefStore, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::RefStore, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((RefStore { index }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::RefStore((*self).clone())
    }
}
