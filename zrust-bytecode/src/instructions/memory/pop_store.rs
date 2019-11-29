use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Removes value from the top of the stack and stores it in the storage.
#[derive(Debug, PartialEq, Clone)]
pub struct PopStore {
    pub index: usize,
}

impl PopStore {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for PopStore {
    fn to_assembly(&self) -> String {
        format!("pop_store {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::PopStore
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::PopStore, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(PopStore, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::PopStore, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((PopStore { index }, len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::PopStore((*self).clone())
    }
}
