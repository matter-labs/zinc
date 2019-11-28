use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::BigInt;

#[deprecated(note = "Use PushConst")]
pub type Push = PushConst;

#[derive(Debug, PartialEq, Clone)]
pub struct PushConst {
    pub value: BigInt,
}

impl PushConst {
    pub fn new(value: BigInt, _signed: bool, _bit_length: usize) -> Self {
        PushConst { value }
    }
}

impl InstructionInfo for PushConst {
    fn to_assembly(&self) -> String {
        format!("push {}", self.value)
    }

    fn code() -> InstructionCode {
        InstructionCode::PushConst
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::PushConst, &self.value)
    }

    fn decode(bytes: &[u8]) -> Result<(PushConst, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::PushConst, bytes)?;

        Ok((PushConst { value }, len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::PushConst((*self).clone())
    }
}
