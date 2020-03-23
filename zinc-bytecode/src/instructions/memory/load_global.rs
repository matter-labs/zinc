use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads value from data stack and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoadGlobal {
    pub address: usize,
}

impl LoadGlobal {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for LoadGlobal {
    fn to_assembly(&self) -> String {
        format!("load_global {}", self.address)
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadGlobal((*self).clone())
    }
}
