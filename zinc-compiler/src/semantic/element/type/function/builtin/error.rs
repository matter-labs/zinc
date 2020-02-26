//!
//! The semantic analyzer standard library function type error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "the number of `dbg!(...)` arguments must be equal to the number of placeholders in the format string: expected {}, but got {}. Example: 'dbg!(\"{{}}, {{}}\", a, b)'",
        _0, _1
    )]
    DebugArgumentCount(usize, usize),
}
