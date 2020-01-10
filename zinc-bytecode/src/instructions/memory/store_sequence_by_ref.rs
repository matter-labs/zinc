use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone)]
pub struct StoreSequenceByRef {
    pub len: usize,
}

impl StoreSequenceByRef {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl InstructionInfo for StoreSequenceByRef {
    fn to_assembly(&self) -> String {
        format!("store_sequence_by_ref {}", self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreSequenceByRef
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequenceByRef((*self).clone())
    }
}
