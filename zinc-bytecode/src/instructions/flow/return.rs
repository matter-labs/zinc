//!
//! The 'function return' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Return {
    pub outputs_count: usize,
}

impl Return {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Return {
    fn into(self) -> Instruction {
        Instruction::Return(self)
    }
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.outputs_count)
    }
}
