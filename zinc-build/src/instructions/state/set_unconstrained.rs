//!
//! The `set unconstrained` state modifier instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

///
/// The `set unconstrained` state modifier instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetUnconstrained;

impl SetUnconstrained {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for SetUnconstrained {
    fn into(self) -> Instruction {
        Instruction::SetUnconstrained(self)
    }
}

impl fmt::Display for SetUnconstrained {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set: unconstrained")
    }
}
