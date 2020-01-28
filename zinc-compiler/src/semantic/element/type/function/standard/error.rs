//!
//! The semantic analyzer function type error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "function '{}' expected {} arguments, but got {}",
        _0, _1, _2
    )]
    ArgumentCount(&'static str, usize, usize),
    #[fail(display = "function '{}' expected type '{}', but got '{}'", _0, _1, _2)]
    ArgumentType(&'static str, String, String),
}
