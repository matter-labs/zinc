use crate::{Instruction, InstructionInfo};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
