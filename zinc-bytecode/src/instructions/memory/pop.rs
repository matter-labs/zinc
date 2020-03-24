use crate::{Instruction, InstructionInfo};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Pop {
    pub count: usize,
}

impl Pop {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl InstructionInfo for Pop {
    fn to_assembly(&self) -> String {
        format!("pop {}", self.count)
    }

    fn wrap(&self) -> Instruction {
        Instruction::Pop((*self).clone())
    }
}
