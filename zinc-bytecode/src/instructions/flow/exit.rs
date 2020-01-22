use crate::instructions::utils;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_traits::ToPrimitive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exit {
    pub outputs_count: usize,
}

impl Exit {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for Exit {
    fn to_assembly(&self) -> String {
        "exit".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Exit
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(Self::code(), &self.outputs_count.into())
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(Self::code(), bytes)?;
        let outputs_count = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(outputs_count), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Exit((*self).clone())
    }
}
