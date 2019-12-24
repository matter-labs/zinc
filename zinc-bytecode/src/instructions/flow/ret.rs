use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub outputs_count: usize,
}

impl Return {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for Return {
    fn to_assembly(&self) -> String {
        format!("ret {}", self.outputs_count)
    }

    fn code() -> InstructionCode {
        InstructionCode::Return
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::Return, &BigInt::from(self.outputs_count))
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::Return, bytes)?;
        let count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(count), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Return((*self).clone())
    }
}
