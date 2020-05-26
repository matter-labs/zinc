use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Instruction;
use crate::InstructionInfo;

/// Loads several values from data stack and pushes them onto evaluation stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Load {
    pub address: usize,
    pub len: usize,
}

impl Load {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for Load {
    fn to_assembly(&self) -> String {
        format!("load_array {} {}", self.address, self.len)
    }

    fn wrap(self) -> Instruction {
        Instruction::Load(self)
    }
}
