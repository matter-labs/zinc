//!
//! The BigInt parsing error.
//!

use std::num::ParseIntError;

use failure::Fail;
use num::bigint::ParseBigIntError;

///
/// The BigInt parsing error.
///
#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    /// The integer or fractional number part is invalid.
    #[fail(display = "number parsing: {}", _0)]
    NumberParsing(ParseBigIntError),
    /// The exponent is invalid.
    #[fail(display = "exponent parsing: {}", _0)]
    ExponentParsing(ParseIntError),
    /// The exponent is lesser than the number of fractional digits.
    #[fail(
        display = "the exponent {} is too small, it must be bigger than the number of fractional digits",
        _0
    )]
    ExponentTooSmall(u32),
}

impl From<ParseBigIntError> for Error {
    fn from(inner: ParseBigIntError) -> Self {
        Self::NumberParsing(inner)
    }
}

impl From<ParseIntError> for Error {
    fn from(inner: ParseIntError) -> Self {
        Self::ExponentParsing(inner)
    }
}
