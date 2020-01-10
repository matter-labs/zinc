use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct LoadByIndexByRef {
    len: usize,
}

impl LoadByIndexByRef {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl InstructionInfo for LoadByIndexByRef {
    fn to_assembly(&self) -> String {
        format!("load_by_index_by_ref {}", self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadByIndexByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadByIndexByRef((*self).clone())
    }
}
