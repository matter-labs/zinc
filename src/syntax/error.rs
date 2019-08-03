//!
//! The syntax error.
//!

use failure::Fail;

use crate::syntax::IdentificatorError;
use crate::syntax::TypeKeywordError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Expected either of: {:?} (got '{}')", _0, _1)]
    Expected(Vec<&'static str>, String),
    #[fail(display = "Invalid variable name '{}': {}", _0, _1)]
    InvalidIdentificator(String, IdentificatorError),
    #[fail(display = "Invalid type keyword '{}': {}", _0, _1)]
    InvalidTypeKeyword(String, TypeKeywordError),
    #[fail(display = "Unexpected end")]
    UnexpectedEnd,
}
