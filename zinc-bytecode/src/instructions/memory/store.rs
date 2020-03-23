use crate::{Instruction, InstructionInfo};


use serde_derive::{Deserialize, Serialize};

/// Stores value from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Store {
    pub index: usize,
}

impl Store {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl InstructionInfo for Store {
    fn to_assembly(&self) -> String {
        format!("store {}", self.index)
    }

    fn wrap(&self) -> Instruction {
        Instruction::Store((*self).clone())
    }
}
