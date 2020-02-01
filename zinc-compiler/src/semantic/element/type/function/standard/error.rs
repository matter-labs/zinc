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
    #[fail(display = "function '{}' expected type '{}', but got '{}'", _0, _1, _2)]
    ArgumentType(&'static str, String, String),
    #[fail(
        display = "attempted to truncate an array from size {} to size {}",
        _0, _1
    )]
    TruncateInvalidLength(usize, usize),
    #[fail(display = "attempted to pad an array from size {} to size {}", _0, _1)]
    PadInvalidLength(usize, usize),
}
