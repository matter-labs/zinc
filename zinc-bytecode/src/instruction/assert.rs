//!
//! The 'assert' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    fn wrap(self) -> Instruction {
        Instruction::Assert(self)
    }
}
