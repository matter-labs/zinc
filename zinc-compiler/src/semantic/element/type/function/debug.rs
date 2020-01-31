//!
//! The semantic analyzer `dbg!` function type element.
//!

use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct DebugInstructionFunction {
    pub identifier: &'static str,
}

impl DebugInstructionFunction {
    pub fn new() -> Self {
        Self { identifier: "dbg" }
    }
}

impl fmt::Display for DebugInstructionFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(format: &str, args: ..)", self.identifier)
    }
}
