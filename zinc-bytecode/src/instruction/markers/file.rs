//!
//! The 'file marker' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMarker {
    pub file: String,
}

impl FileMarker {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

impl InstructionInfo for FileMarker {
    fn to_assembly(&self) -> String {
        format!("marker: file = \"{}\"", self.file)
    }

    fn wrap(self) -> Instruction {
        Instruction::FileMarker(self)
    }
}
