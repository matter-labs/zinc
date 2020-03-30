//!
//! The semantic analyzer built-in function element.
//!

mod tests;

pub mod assert;
pub mod debug;
pub mod error;

use std::fmt;

use self::assert::Function as AssertFunction;
use self::debug::Function as DebugFunction;

#[derive(Debug, Clone)]
pub enum Function {
    Assert(AssertFunction),
    Debug(DebugFunction),
}

impl Function {
    pub fn new_assert() -> Self {
        Self::Assert(AssertFunction::new())
    }

    pub fn new_debug() -> Self {
        Self::Debug(DebugFunction::new())
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Assert(inner) => inner.identifier(),
            Self::Debug(inner) => inner.identifier(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assert(inner) => write!(f, "{}", inner),
            Self::Debug(inner) => write!(f, "{}", inner),
        }
    }
}
