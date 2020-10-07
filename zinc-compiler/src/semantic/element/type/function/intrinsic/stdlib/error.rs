//!
//! The semantic analyzer standard library function error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer standard library function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An array is tried to be truncated to a bigger size. The `pad` function must be used instead.
    ArrayTruncatingToBiggerSize {
        /// The error location data.
        location: Location,
        /// The original lesser array size.
        from: usize,
        /// The new invalid bigger array size.
        to: usize,
    },
    /// An array is tried to be padded to a lesser size. The `truncate` function must be used instead.
    ArrayPaddingToLesserSize {
        /// The error location data.
        location: Location,
        /// The original bigger array size.
        from: usize,
        /// The new invalid lesser array size.
        to: usize,
    },
    /// The new length value cannot be converted to `usize` type.
    ArrayNewLengthInvalid {
        /// The error location data.
        location: Location,
        /// The stringified new length argument value.
        value: String,
    },
}
