//!
//! The 'function return' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Return {
    pub outputs_count: usize,
}

impl Return {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for Return {
    fn to_assembly(&self) -> String {
        format!("ret {}", self.outputs_count)
    }

    fn wrap(self) -> Instruction {
        Instruction::Return(self)
    }
}
