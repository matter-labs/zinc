use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct Slice {
    pub array_len: usize,
    pub slice_len: usize,
}

impl Slice {
    pub fn new(array_len: usize, slice_len: usize) -> Self {
        Self {
            array_len,
            slice_len,
        }
    }
}

impl InstructionInfo for Slice {
    fn to_assembly(&self) -> String {
        format!("slice {} {}", self.array_len, self.slice_len)
    }

    fn code() -> InstructionCode {
        InstructionCode::Slice
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.array_len, self.slice_len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;
        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        self.array_len
    }

    fn outputs_count(&self) -> usize {
        self.slice_len
    }

    fn wrap(&self) -> Instruction {
        Instruction::Slice((*self).clone())
    }
}
