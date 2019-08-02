//!
//! The syntax error.
//!

use failure::Fail;

use crate::syntax::VariableNameError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Expected either of: {:?} (got '{}')", _0, _1)]
    Expected(Vec<&'static str>, String),
    #[fail(display = "Invalid variable name '{}': {}", _0, _1)]
    InvalidVariableName(String, VariableNameError),
    #[fail(display = "Unexpected end")]
    UnexpectedEnd,
}
