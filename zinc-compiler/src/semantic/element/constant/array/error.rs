//!
//! The semantic analyzer constant array element error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer constant array element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The subsequent element type is not equal to the first element type, which dictates the array element type.
    PushingInvalidType {
        /// The error location data.
        location: Location,
        /// The expected array type, which is dictated by the first element pushed.
        expected: String,
        /// The invalid array element, which is actually found.
        found: String,
    },
    /// The constant array index is out of the compile time-known range.
    IndexOutOfRange {
        /// The error location data.
        location: Location,
        /// The invalid array index, which is actually found.
        index: String,
        /// The actual array size, which is violated by `index`.
        size: usize,
    },
    /// The slice left bound is negative.
    SliceStartOutOfRange {
        /// The error location data.
        location: Location,
        /// The left slice bound as string.
        start: String,
    },
    /// The constant right range bound is out of the compile time-known range.
    SliceEndOutOfRange {
        /// The error location data.
        location: Location,
        /// The right slice bound as string.
        end: String,
        /// The actual array size, which is violated by `end`.
        size: usize,
    },
    /// The array slicing range left bound must be not be bigger than the right one.
    SliceEndLesserThanStart {
        /// The error location data.
        location: Location,
        /// The left slice bound as string.
        start: String,
        /// The right slice bound as string.
        end: String,
    },
}
