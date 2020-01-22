use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequenceByIndexByRef {
    pub value_len: usize,
    pub array_len: usize,
}

impl StoreSequenceByIndexByRef {
    pub fn new(value_len: usize, array_len: usize) -> Self {
        Self {
            value_len,
            array_len,
        }
    }
}

impl InstructionInfo for StoreSequenceByIndexByRef {
    fn to_assembly(&self) -> String {
        "store_sequence_by_index_by_ref".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreSequenceByIndexByRef
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
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequenceByIndexByRef((*self).clone())
    }
}
