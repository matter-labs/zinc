//!
//! The interpreter place error.
//!

use failure::Fail;

use crate::interpreter::Value;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "assignment types mismatch: [{}] and [{}]", _0, _1)]
    AssignmentTypesMismatch(Value, Value),
    #[fail(display = "indexing not an array, but: [{}]", _0)]
    IndexingNotArray(Value),
    #[fail(display = "indexing expected an integer, but got: [{}]", _0)]
    IndexingExpectedInteger(Value),
    #[fail(display = "index {} is out of range", _0)]
    IndexOutOfRange(usize),
}
