//!
//! The 'program exit' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exit {
    pub outputs_count: usize,
}

impl Exit {
    pub fn new(outputs_count: usize) -> Self {
        Self { outputs_count }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Exit {
    fn into(self) -> Instruction {
        Instruction::Exit(self)
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exit")
    }
}
