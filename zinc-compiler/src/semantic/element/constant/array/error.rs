//!
//! The semantic analyzer constant array element error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    PushingInvalidType {
        location: Location,
        expected: String,
        found: String,
    },
    IndexOutOfRange {
        location: Location,
        index: String,
        size: usize,
    },
    SliceStartOutOfRange {
        location: Location,
        start: String,
    },
    SliceEndOutOfRange {
        location: Location,
        end: String,
        size: usize,
    },
    SliceEndLesserThanStart {
        location: Location,
        start: String,
        end: String,
    },
}
