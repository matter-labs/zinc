//!
//! The standard library function identifier.
//!

pub mod function_identifier;

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

use self::function_identifier::FunctionIdentifier;

///
/// The `standard library function call` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallStd {
    /// The standard library function identifier.
    pub identifier: FunctionIdentifier,
    /// The input size in field elements.
    pub input_size: usize,
    /// The output size in field elements.
    pub output_size: usize,
}

impl CallStd {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(identifier: FunctionIdentifier, input_size: usize, output_size: usize) -> Self {
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

impl Into<Instruction> for CallStd {
    fn into(self) -> Instruction {
        Instruction::CallStd(self)
    }
}

impl fmt::Display for CallStd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "call_std {:?}({}) -> {}",
            self.identifier, self.input_size, self.output_size
        )
    }
}
