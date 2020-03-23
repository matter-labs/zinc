use crate::{Instruction, InstructionInfo};


use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    fn wrap(&self) -> Instruction {
        Instruction::Return((*self).clone())
    }
}
