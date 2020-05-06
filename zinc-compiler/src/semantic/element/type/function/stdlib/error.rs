//!
//! The semantic analyzer standard library function error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArrayTruncatingToBiggerSize {
        location: Location,
        from: usize,
        to: usize,
    },
    ArrayPaddingToLesserSize {
        location: Location,
        from: usize,
        to: usize,
    },
    ArrayNewLengthInvalid {
        location: Location,
        value: String,
    },
}
