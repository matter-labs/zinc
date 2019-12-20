use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Ref {
    pub address: usize,
}

impl Ref {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for Ref {
    fn to_assembly(&self) -> String {
        format!("ref {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::Ref
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(InstructionCode::Ref, &self.address.to_bigint().unwrap())
    }

    fn decode(bytes: &[u8]) -> Result<(Ref, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(InstructionCode::Ref, bytes)?;
        let count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Ref { address: count }, len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Ref((*self).clone())
    }
}
