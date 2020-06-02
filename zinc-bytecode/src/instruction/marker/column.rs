//!
//! The 'column marker' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnMarker {
    pub column: usize,
}

impl ColumnMarker {
    pub fn new(column: usize) -> Self {
        Self { column }
    }

    pub fn is_debug(&self) -> bool {
        true
    }
}

impl Into<Instruction> for ColumnMarker {
    fn into(self) -> Instruction {
        Instruction::ColumnMarker(self)
    }
}

impl fmt::Display for ColumnMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: column = \"{}\"", self.column)
    }
}
