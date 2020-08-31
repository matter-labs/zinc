//!
//! The integer type inference error.
//!

use failure::Fail;
use num_bigint::BigInt;

///
/// The type inference error.
///
#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    /// The value is larger then the maximal allowed value.
    #[fail(
        display = "overflow: value `{}` is out of range of bitlength {} with sign `{}`",
        value, bitlength, is_signed
    )]
    Overflow {
        /// The invalid value.
        value: BigInt,
        /// Whether the type is signed.
        is_signed: bool,
        /// The maximal allowed bitlength.
        bitlength: usize,
    },
}
