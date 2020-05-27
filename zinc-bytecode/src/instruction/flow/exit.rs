//!
//! The 'program exit' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exit {
    pub outputs_count: usize,
}

impl Exit {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }
}

impl InstructionInfo for Exit {
    fn to_assembly(&self) -> String {
        "exit".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Exit(self)
    }
}
