use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    fn wrap(&self) -> Instruction {
        Instruction::FunctionMarker((*self).clone())
    }
}
