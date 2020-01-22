use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreByIndexByRef {
    pub len: usize,
}

impl StoreByIndexByRef {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl InstructionInfo for StoreByIndexByRef {
    fn to_assembly(&self) -> String {
        format!("store_by_index_by_ref {}", self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreByIndexByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreByIndexByRef((*self).clone())
    }
}
