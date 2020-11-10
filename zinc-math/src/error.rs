//!
//! The type inference error.
//!

use num::BigInt;
use thiserror::Error;

///
/// The type inference error.
///
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// The BigInt parsing error.
    #[error("integer value parsing: {0}")]
    NumberParsing(#[from] num::bigint::ParseBigIntError),
    /// The integer parsing error.
    #[error("exponent parsing: {0}")]
    ExponentParsing(#[from] std::num::ParseIntError),
    /// The integer value is too large for the inferred type.
    #[error(
        "the value `{value}` is out of range of bitlength {bitlength} with sign `{is_signed}`"
    )]
    Overflow {
        /// The invalid value.
        value: BigInt,
        /// Whether the type is signed.
        is_signed: bool,
        /// The maximal allowed bitlength.
        bitlength: usize,
    },
    /// The exponent is lesser than the number of fractional digits.
    #[error(
        "the exponent {0} is too small, as it must be bigger than the number of fractional digits"
    )]
    ExponentTooSmall(u32),
}
