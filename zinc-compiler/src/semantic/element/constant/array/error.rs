//!
//! The semantic analyzer constant array element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    PushingInvalidType { expected: String, found: String },
    IndexOutOfRange { index: String, size: usize },
    SliceStartOutOfRange { start: String },
    SliceEndOutOfRange { end: String, size: usize },
    SliceEndLesserThanStart { start: String, end: String },
}
