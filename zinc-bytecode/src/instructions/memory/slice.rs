use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct Slice {
    pub len: usize,
    pub slice_len: usize,
    pub slice_offset: usize,
}

impl Slice {
    pub fn new(len: usize, slice_len: usize, slice_offset: usize) -> Self {
        Self {
            len,
            slice_len,
            slice_offset
        }
    }
}

impl InstructionInfo for Slice {
    fn to_assembly(&self) -> String {
        format!("slice {} {} {}", self.len, self.slice_len, self.slice_offset)
    }

    fn code() -> InstructionCode {
        InstructionCode::Slice
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.len, self.slice_len, self.slice_offset])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 3)?;
        Ok((Self::new(args[0], args[1], args[2]), len))
    }

    fn inputs_count(&self) -> usize {
        self.len
    }

    fn outputs_count(&self) -> usize {
        self.slice_len
    }

    fn wrap(&self) -> Instruction {
        Instruction::Slice((*self).clone())
    }
}
