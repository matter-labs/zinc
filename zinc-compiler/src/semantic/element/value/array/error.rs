//!
//! The semantic analyzer array value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    PushingInvalidType(String, String),
    SliceStartOutOfRange(String),
    SliceEndOutOfRange(String, String),
    SliceEndLesserThanStart(String, String),
}
