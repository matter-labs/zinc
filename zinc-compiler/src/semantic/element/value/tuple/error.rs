//!
//! The semantic analyzer array value element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInTuple(usize, String), // TODO
}
