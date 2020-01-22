use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PushConst {
    pub value: BigInt,
    pub is_signed: bool,
    pub bit_length: usize,
}

impl PushConst {
    pub fn new(value: BigInt, is_signed: bool, bit_length: usize) -> Self {
        Self {
            value,
            is_signed,
            bit_length,
        }
    }

    pub fn new_untyped(value: BigInt) -> Self {
        // TODO: Remove this constructor, it is for testing only.
        Self {
            value,
            is_signed: true,
            bit_length: 254,
        }
    }
}

impl InstructionInfo for PushConst {
    fn to_assembly(&self) -> String {
        format!(
            "push {}: {}{}",
            self.value,
            if self.is_signed { "i" } else { "u" },
            self.bit_length,
        )
    }

    fn code() -> InstructionCode {
        InstructionCode::PushConst
    }

    fn encode(&self) -> Vec<u8> {
        let args = [
            self.value.clone(),
            BigInt::from(self.is_signed as usize),
            BigInt::from(self.bit_length),
        ];
        utils::encode_with_args(InstructionCode::PushConst, &args)
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (mut args, len) = utils::decode_with_bigint_args(Self::code(), bytes, 3)?;

        let bit_length = args
            .pop()
            .unwrap()
            .to_usize()
            .ok_or(DecodingError::ConstantTooLong)?;
        let is_signed = match args.pop().unwrap().to_u8() {
            Some(b) if b == 0 => Ok(false),
            Some(b) if b == 1 => Ok(true),
            _ => Err(DecodingError::ConstantTooLong),
        }?;
        let value = args.pop().unwrap();

        Ok((Self::new(value, is_signed, bit_length), len))
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
