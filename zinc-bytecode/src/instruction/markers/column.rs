//!
//! The 'column marker' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
