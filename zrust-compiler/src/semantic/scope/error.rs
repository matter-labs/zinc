//!
//! The semantic analyzer scope error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    UndeclaredItem(String),
    #[fail(display = "undeclared variable '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "undeclared type '{}'", _0)]
    UndeclaredType(String),
    #[fail(display = "redeclared item '{}'", _0)]
    RedeclaredItem(String),
    #[fail(display = "mutating an immutable object '{}'", _0)]
    MutatingImmutable(String),
    #[fail(
        display = "assigning a value of type '{}' to a variable of type '{}'",
        _0, _1
    )]
    VariableTypeMismatch(TypeVariant, TypeVariant),
}
