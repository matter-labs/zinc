//!
//! The transpiler error.
//!

use failure::Fail;

use crate::scope::Error as ScopeError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "scope: {}", _0)]
    Scope(ScopeError),
}
