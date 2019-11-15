use crate::{Instruction, InstructionCode, DecodingError, utils};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct Call {
    pub address: usize,
}

impl Instruction for Call {
    fn to_assembly(&self) -> String {
        "call".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Call
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Call, &BigInt::from(self.address))
    }
}

impl Call {
    pub fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Call, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Call, bytes)?;
        let address = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Call::new(address), len))
    }
}
