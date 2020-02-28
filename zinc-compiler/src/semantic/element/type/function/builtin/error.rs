//!
//! The semantic analyzer built-in function error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown(String),
    SpecifierMissing(&'static str),
    DebugArgumentCount(usize, usize),
}
