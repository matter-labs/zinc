use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct LoadSequenceByIndexByRef {
    pub value_len: usize,
    pub array_len: usize,
}

impl LoadSequenceByIndexByRef {
    pub fn new(value_len: usize, array_len: usize) -> Self {
        Self { value_len, array_len }
    }
}

impl InstructionInfo for LoadSequenceByIndexByRef {
    fn to_assembly(&self) -> String {
        format!("load_sequence_by_index_by_ref {} {}", self.value_len, self.array_len)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadSequenceByIndexByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.value_len, self.array_len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequenceByIndexByRef((*self).clone())
    }
}
