//!
//! The interpreter scope error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared variable '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "redeclared variable '{}'", _0)]
    RedeclaredVariable(String),
    #[fail(display = "mutating an immutable variable '{}'", _0)]
    MutatingImmutableVariable(String),
    #[fail(
        display = "the value being assigned has type '{}', but expected '{}'",
        _0, _1
    )]
    AssignmentInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "indexing a not-array variable '{}'", _0)]
    IndexingNotArray(String),
    #[fail(
        display = "index {} is out of range of the array variable '{}'",
        _0, _1
    )]
    ArrayIndexOutOfRange(usize, String),
}
