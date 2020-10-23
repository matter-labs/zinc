//!
//! The `function marker` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

///
/// The `function marker` debug instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionMarker {
    /// The source code function name.
    pub function: String,
}

impl FunctionMarker {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(function: String) -> Self {
        Self { function }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        true
    }
}

impl Into<Instruction> for FunctionMarker {
    fn into(self) -> Instruction {
        Instruction::FunctionMarker(self)
    }
}

impl fmt::Display for FunctionMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker: function = \"{}\"", self.function)
    }
}
