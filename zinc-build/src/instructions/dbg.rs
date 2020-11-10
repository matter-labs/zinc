//!
//! The `debug` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::Type;
use crate::instructions::Instruction;

///
/// The `debug` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dbg {
    /// The format string with `{}` placeholders.
    pub format: String,
    /// The types of the values that must be formatted by the instruction execution environment.
    /// The types help the environment to choose the way of formatting.
    pub argument_types: Vec<Type>,
}

impl Dbg {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(format: String, arg_types: Vec<Type>) -> Self {
        Self {
            format,
            argument_types: arg_types,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Dbg {
    fn into(self) -> Instruction {
        Instruction::Dbg(self)
    }
}

impl fmt::Display for Dbg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dbg")
    }
}
