//!
//! The syntax error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::syntax::IdentifierError;
use crate::syntax::TypeBuilderError;
use crate::syntax::TypeKeywordError;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "expected either of: {:?} (got '{}')", _0, _1)]
    Expected(Vec<&'static str>, String),
    #[fail(display = "invalid variable name '{}': {}", _0, _1)]
    InvalidIdentifier(String, IdentifierError),
    #[fail(display = "invalid type keyword '{}': {}", _0, _1)]
    InvalidTypeKeyword(String, TypeKeywordError),
    #[fail(display = "invalid type: {}", _0)]
    InvalidType(TypeBuilderError),
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
}
