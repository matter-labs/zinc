use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
