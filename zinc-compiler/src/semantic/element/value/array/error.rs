//!
//! The semantic analyzer array value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    PushingInvalidType { expected: String, found: String },
    SliceStartOutOfRange { start: String },
    SliceEndOutOfRange { end: String, size: usize },
    SliceEndLesserThanStart { start: String, end: String },
}
