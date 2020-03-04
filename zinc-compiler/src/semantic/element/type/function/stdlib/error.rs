//!
//! The semantic analyzer standard library function error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    ArrayTruncatingToBiggerSize { from: usize, to: usize },
    ArrayPaddingToLesserSize { from: usize, to: usize },
    ArrayNewLengthInvalid { value: String },
}

impl Error {
    pub fn array_truncating_to_bigger_size(from: usize, to: usize) -> Self {
        Self::ArrayTruncatingToBiggerSize { from, to }
    }

    pub fn array_padding_to_lesser_size(from: usize, to: usize) -> Self {
        Self::ArrayPaddingToLesserSize { from, to }
    }

    pub fn array_new_length_invalid(value: String) -> Self {
        Self::ArrayNewLengthInvalid { value }
    }
}
