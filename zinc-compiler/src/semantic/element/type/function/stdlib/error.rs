//!
//! The semantic analyzer standard library function type error.
//!

use failure::Fail;

use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "attempted to truncate an array from the size {} to the bigger size {}",
        _0, _1
    )]
    TruncatingToBiggerSize(usize, usize),
    #[fail(
        display = "attempted to pad an array from the size {} to the bigger size {}",
        _0, _1
    )]
    PaddingToBiggerSize(usize, usize),
    #[fail(display = "new array length '{}' cannot act as an index: {}", _0, _1)]
    NewArrayLengthInvalid(String, IntegerConstantError),
}
