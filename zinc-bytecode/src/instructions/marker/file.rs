//!
//! The 'file marker' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMarker {
    pub file: String,
}

impl FileMarker {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn is_debug(&self) -> bool {
        true
    }
}

impl Into<Instruction> for FileMarker {
    fn into(self) -> Instruction {
        Instruction::FileMarker(self)
    }
}

impl fmt::Display for FileMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: file = \"{}\"", self.file)
    }
}
