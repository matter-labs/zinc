use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq, Clone)]
//#[deprecated(note = "Use storage instead!")]
pub struct CopyGlobal {
    pub index: usize,
}

impl CopyGlobal {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for CopyGlobal {
    fn to_assembly(&self) -> String {
        format!("copy_global {}", self.index)
    }

    fn code() -> InstructionCode {
        InstructionCode::CopyGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::CopyGlobal, &self.index.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(CopyGlobal, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::CopyGlobal, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((CopyGlobal { index }, len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::CopyGlobal((*self).clone())
    }
}
