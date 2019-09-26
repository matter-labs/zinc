//!
//! The interpreter scope error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared variable: '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "redeclared variable: '{}'", _0)]
    RedeclaredVariable(String),
    #[fail(display = "mutating an immutable variable: '{}'", _0)]
    MutatingImmutableVariable(String),
}
