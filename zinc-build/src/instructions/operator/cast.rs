//!
//! The `cast` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::instructions::Instruction;

///
/// The `cast` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cast {
    /// The type casted to.
    pub r#type: ScalarType,
}

impl Cast {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(r#type: ScalarType) -> Self {
        Self { r#type }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Cast {
    fn into(self) -> Instruction {
        Instruction::Cast(self)
    }
}

impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cast {}", self.r#type)
    }
}
