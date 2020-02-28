//!
//! The semantic analyzer array value element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "a value of type '{}' cannot into an array of type '{}'",
        _0, _1
    )]
    PushingInvalidType(String, String),
    #[fail(display = "left range bound {} cannot be negative", _0)]
    SliceStartOutOfRange(String),
    #[fail(
        display = "right range bound {} is out of range of the array of size {}",
        _0, _1
    )]
    SliceEndOutOfRange(String, String),
    #[fail(
        display = "right range bound {} is lesser than the left one {}",
        _0, _1
    )]
    SliceEndLesserThanStart(String, String),
}
