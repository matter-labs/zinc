//!
//! The standard library function identifier.
//!

pub mod function_identifier;

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::instructions::Instruction;

use self::function_identifier::LibraryFunctionIdentifier;

///
/// The `standard library function call` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallLibrary {
    /// The standard library function identifier.
    pub identifier: LibraryFunctionIdentifier,
    /// The input size in field elements.
    pub input_size: usize,
    /// The output size in field elements.
    pub output_size: usize,
}

impl CallLibrary {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: LibraryFunctionIdentifier,
        input_size: usize,
        output_size: usize,
    ) -> Self {
        Self {
            identifier,
            input_size,
            output_size,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for CallLibrary {
    fn into(self) -> Instruction {
        Instruction::CallLibrary(self)
    }
}

impl fmt::Display for CallLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "call_library {:?}({}) -> {}",
            self.identifier, self.input_size, self.output_size
        )
    }
}
