//!
//! The semantic analyzer array value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDoesNotExist(usize, String),
}
