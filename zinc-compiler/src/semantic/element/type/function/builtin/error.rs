//!
//! The semantic analyzer standard library function type error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "function '{}' expected {} arguments, but got {}",
        _0, _1, _2
    )]
    ArgumentCount(&'static str, usize, usize),
    #[fail(
        display = "function '{}' expected type '{}' as the argument {}, but got '{}'",
        _0, _1, _2, _3
    )]
    ArgumentType(&'static str, String, usize, String),
    #[fail(
        display = "function '{}' expected a constant as the argument {}, but got a non-constant value '{}'",
        _0, _1, _2
    )]
    ArgumentConstantness(&'static str, usize, String),
    #[fail(
        display = "function '{}' expected an evaluable as the argument {}, but got '{}'",
        _0, _1, _2
    )]
    ArgumentNotEvaluable(&'static str, usize, String),
    #[fail(
        display = "the number of {}!(...) arguments must be equal to the number of placeholders in the format string: expected {}, but got {}. Example: 'dbg!(\"{{}}, {{}}\", a, b)'",
        _0, _1, _2
    )]
    DebugArgumentCount(&'static str, usize, usize),
}
