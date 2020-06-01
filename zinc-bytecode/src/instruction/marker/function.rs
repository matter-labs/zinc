//!
//! The 'function marker' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionMarker {
    pub function: String,
}

impl FunctionMarker {
    pub fn new(function: String) -> Self {
        Self { function }
    }

    pub fn is_debug(&self) -> bool {
        true
    }

    pub fn wrap(self) -> Instruction {
        Instruction::FunctionMarker(self)
    }
}

impl fmt::Display for FunctionMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: function = \"{}\"", self.function)
    }
}
