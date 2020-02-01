use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadSequenceByRef {
    pub value_len: usize,
    pub array_len: usize,
}

impl LoadSequenceByRef {
    pub fn new(value_len: usize, array_len: usize) -> Self {
        Self {
            value_len,
            array_len,
        }
    }
}

impl InstructionInfo for LoadSequenceByRef {
    fn to_assembly(&self) -> String {
        format!("load_sequence_by_ref {} {}", self.value_len, self.array_len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadSequenceByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.value_len, self.array_len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        self.value_len
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequenceByRef((*self).clone())
    }
}
