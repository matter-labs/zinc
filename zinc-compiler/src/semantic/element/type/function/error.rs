//!
//! The semantic analyzer standard library function type error.
//!

use failure::Fail;

use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "function '{}' expected {} arguments, but got {}",
        _0, _1, _2
    )]
    ArgumentCount(String, usize, usize),
    #[fail(
        display = "function '{}' expected type '{}' as the argument {} ('{}'), but got '{}'",
        _0, _1, _2, _3, _4
    )]
    ArgumentType(String, String, usize, String, String),
    #[fail(
        display = "function '{}' expected a constant as the argument {}, but got a non-constant value of type '{}'",
        _0, _1, _2
    )]
    ArgumentConstantness(String, usize, String),
    #[fail(
        display = "function '{}' expected an evaluable as the argument {}, but got '{}'",
        _0, _1, _2
    )]
    ArgumentNotEvaluable(String, usize, String),
    #[fail(
        display = "function '{}' must return a value of type '{}', but got '{}'",
        _0, _1, _2
    )]
    ReturnType(String, String, String),
    #[fail(display = "calling a non-callable object '{}'", _0)]
    NonCallableObject(String),
    #[fail(display = "unknown built-in function '{}' is unknown", _0)]
    BuiltInUnknown(String),
    #[fail(
        display = "built-in function '{0}' must be called with '!', e.g. `{0}!(...)`",
        _0
    )]
    BuiltInSpecifierMissing(&'static str),

    #[fail(display = "{}", _0)]
    BuiltIn(BuiltInFunctionError),
    #[fail(display = "{}", _0)]
    StandardLibrary(StandardLibraryFunctionError),
}
