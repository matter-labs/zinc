//!
//! Casting error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "casting from invalid type: from '{}' to '{}'", _0, _1)]
    FromInvalidType(String, String),
    #[fail(display = "casting to invalid type: from '{}' to '{}'", _0, _1)]
    ToInvalidType(String, String),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    DataLossPossible(usize, usize),
}
