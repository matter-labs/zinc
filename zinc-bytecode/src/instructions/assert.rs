use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Assert {
    pub message: Option<String>,
}

impl Assert {
    pub fn new(message: Option<String>) -> Self {
        Self { message }
    }
}

impl InstructionInfo for Assert {
    fn to_assembly(&self) -> String {
        match &self.message {
            None => "assert".to_owned(),
            Some(text) => format!("assert \"{}\"", text),
        }
    }

    fn wrap(&self) -> Instruction {
        Instruction::Assert((*self).clone())
    }
}
