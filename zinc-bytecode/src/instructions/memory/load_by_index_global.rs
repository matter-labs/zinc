use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

/// Takes `index` from evaluation stack, loads value from data stack from `address + index` onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadByIndexGlobal {
    pub address: usize,
    pub len: usize,
}

impl LoadByIndexGlobal {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadByIndexGlobal {
    fn to_assembly(&self) -> String {
        format!("load_by_index_global {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadByIndexGlobal
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.address, self.len])
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
        Instruction::LoadByIndexGlobal((*self).clone())
    }
}
