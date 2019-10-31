//!
//! The semantic analyzer element value error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "operator '{}' expected a boolean value, but got '{}'",
        _0, _1
    )]
    ExpectedBoolean(&'static str, TypeVariant),
    #[fail(
        display = "operator '{}' expected an integer value, but got '{}'",
        _0, _1
    )]
    ExpectedInteger(&'static str, TypeVariant),
}
