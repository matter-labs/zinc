//!
//! The semantic analyzer intrinsic function element.
//!

#[cfg(test)]
mod tests;

pub mod assert;
pub mod debug;
pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;

use self::assert::Function as AssertFunction;
use self::debug::Function as DebugFunction;

///
/// The semantic analyzer intrinsic function element.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `assert!(...)` function. See the inner element description.
    Assert(AssertFunction),
    /// The `dbg!(...)` function. See the inner element description.
    Debug(DebugFunction),
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_assert() -> Self {
        Self::Assert(AssertFunction::new())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_debug() -> Self {
        Self::Debug(DebugFunction::new())
    }

    ///
    /// Returns the function identifier, which is known at compile time.
    ///
    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Assert(inner) => inner.identifier,
            Self::Debug(inner) => inner.identifier,
        }
    }

    ///
    /// Sets the function call location in the code.
    ///
    pub fn set_location(&mut self, location: Location) {
        match self {
            Self::Assert(inner) => inner.location = Some(location),
            Self::Debug(inner) => inner.location = Some(location),
        }
    }

    ///
    /// Returns the location of the function call.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Assert(inner) => inner.location,
            Self::Debug(inner) => inner.location,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assert(inner) => write!(f, "{}", inner),
            Self::Debug(inner) => write!(f, "{}", inner),
        }
    }
}
