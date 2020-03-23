use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Loads value from data stack and pushes it onto evaluation stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Load {
    pub address: usize,
}

impl Load {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for Load {
    fn to_assembly(&self) -> String {
        format!("load {}", self.address)
    }

    fn wrap(&self) -> Instruction {
        Instruction::Load((*self).clone())
    }
}
