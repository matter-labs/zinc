use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    fn wrap(&self) -> Instruction {
        Instruction::Slice((*self).clone())
    }
}
