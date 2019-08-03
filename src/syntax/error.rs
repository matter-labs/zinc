//!
//! The syntax error.
//!

use failure::Fail;

use crate::syntax::TypeKeywordError;
use crate::syntax::VariableNameError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Expected either of: {:?} (got '{}')", _0, _1)]
    Expected(Vec<&'static str>, String),
    #[fail(display = "Invalid variable name '{}': {}", _0, _1)]
    InvalidVariableName(String, VariableNameError),
    #[fail(display = "Invalid type keyword '{}': {}", _0, _1)]
    InvalidTypeKeyword(String, TypeKeywordError),
    #[fail(display = "Unexpected end")]
    UnexpectedEnd,
}
