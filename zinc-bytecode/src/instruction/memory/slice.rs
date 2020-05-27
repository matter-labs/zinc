//!
//! The 'evaluation stack slice' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    fn wrap(self) -> Instruction {
        Instruction::Slice(self)
    }
}
