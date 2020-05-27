//!
//! The 'function marker' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionMarker {
    pub function: String,
}

impl FunctionMarker {
    pub fn new(function: String) -> Self {
        Self { function }
    }
}

impl InstructionInfo for FunctionMarker {
    fn to_assembly(&self) -> String {
        format!("marker: function = \"{}\"", self.function)
    }

    fn wrap(self) -> Instruction {
        Instruction::FunctionMarker(self)
    }
}
