//!
//! The 'evaluation stack slice' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

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

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Slice {
    fn into(self) -> Instruction {
        Instruction::Slice(self)
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slice {} {}", self.array_len, self.slice_len)
    }
}
