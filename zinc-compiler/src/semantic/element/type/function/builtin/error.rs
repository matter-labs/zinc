//!
//! The semantic analyzer built-in function error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown { function: String },
    SpecifierMissing { function: &'static str },
    DebugArgumentCount { expected: usize, found: usize },
}

impl Error {
    pub fn unknown(function: String) -> Self {
        Self::Unknown { function }
    }

    pub fn specifier_missing(function: &'static str) -> Self {
        Self::SpecifierMissing { function }
    }

    pub fn debug_argument_count(expected: usize, found: usize) -> Self {
        Self::DebugArgumentCount { expected, found }
    }
}
