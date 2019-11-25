use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Pop {
    pub count: usize,
}

impl Pop {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl InstructionInfo for Pop {
    fn to_assembly(&self) -> String {
        format!("pop {}", self.count)
    }

    fn code() -> InstructionCode {
        InstructionCode::Pop
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Pop, &self.count.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(Pop, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Pop, bytes)?;
        let count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Pop { count }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Pop((*self).clone())
    }
}
