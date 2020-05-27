//!
//! The 'line marker' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineMarker {
    pub line: usize,
}

impl LineMarker {
    pub fn new(line: usize) -> Self {
        Self { line }
    }
}

impl InstructionInfo for LineMarker {
    fn to_assembly(&self) -> String {
        format!("marker: line = \"{}\"", self.line)
    }

    fn wrap(self) -> Instruction {
        Instruction::LineMarker(self)
    }
}
