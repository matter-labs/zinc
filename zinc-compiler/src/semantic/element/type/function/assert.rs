//!
//! The semantic analyzer `assert!` function type element.
//!

use std::fmt;

#[derive(Debug, Clone)]
pub struct AssertInstructionFunction {
    pub identifier: &'static str,
}

impl AssertInstructionFunction {
    pub fn new() -> Self {
        Self {
            identifier: "assert",
        }
    }
}

impl fmt::Display for AssertInstructionFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(condition: bool, message: &str)", self.identifier)
    }
}
