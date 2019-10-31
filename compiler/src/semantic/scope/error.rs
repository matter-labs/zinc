//!
//! The semantic analyzer scope error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    UndeclaredItem(String),
    #[fail(display = "redeclared item '{}'", _0)]
    RedeclaredItem(String),

    #[fail(display = "mutating an immutable object '{}'", _0)]
    MutatingImmutable(String),
    #[fail(
        display = "the value being assigned has type '{}', but expected '{}'",
        _0, _1
    )]
    AssignmentInvalidType(TypeVariant, TypeVariant),
}
