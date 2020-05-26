use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ColumnMarker {
    pub column: usize,
}

impl ColumnMarker {
    pub fn new(column: usize) -> Self {
        Self { column }
    }
}

impl InstructionInfo for ColumnMarker {
    fn to_assembly(&self) -> String {
        format!("marker: column = \"{}\"", self.column)
    }

    fn wrap(self) -> Instruction {
        Instruction::ColumnMarker(self)
    }
}
