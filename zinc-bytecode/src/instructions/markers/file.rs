use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
        format!("marker: directory.file = \"{}\"", self.file)
    }

    fn wrap(&self) -> Instruction {
        Instruction::FileMarker((*self).clone())
    }
}
