//!
//! The semantic analyzer standard library function error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    ArrayTruncatingToBiggerSize(usize, usize),
    ArrayPaddingToLesserSize(usize, usize),
    ArrayNewLengthInvalid(String),
}
