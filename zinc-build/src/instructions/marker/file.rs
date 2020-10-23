//!
//! The `file marker` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `file marker` debug instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMarker {
    /// The source code file path.
    pub file: String,
}

impl FileMarker {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(file: String) -> Self {
        Self { file }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
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
