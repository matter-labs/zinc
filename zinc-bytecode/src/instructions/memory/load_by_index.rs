use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Takes `index` from evaluation stack, loads value from data stack from `address + index` onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadByIndex {
    pub address: usize,
    pub len: usize,
}

impl LoadByIndex {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for LoadByIndex {
    fn to_assembly(&self) -> String {
        format!("load_by_index {} {}", self.address, self.len)
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadByIndex((*self).clone())
    }
}
