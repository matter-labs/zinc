//!
//! The `unset unconstrained` state modifier instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

///
/// The `unset unconstrained` state modifier instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsetUnconstrained;

impl UnsetUnconstrained {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for UnsetUnconstrained {
    fn into(self) -> Instruction {
        Instruction::UnsetUnconstrained(self)
    }
}

impl fmt::Display for UnsetUnconstrained {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unset: unconstrained")
    }
}
