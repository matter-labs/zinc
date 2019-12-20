use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct RefStoreSequence {
    pub index: usize,
}

impl RefStoreSequence {
    pub fn new(index: usize, _len: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for RefStoreSequence {
    fn to_assembly(&self) -> String {
        format!("ref_store_sequence")
    }

    fn code() -> InstructionCode {
        InstructionCode::RefStoreSequence
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::RefStoreSequence, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(RefStoreSequence, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::RefStoreSequence, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((RefStoreSequence { index }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::RefStoreSequence((*self).clone())
    }
}
