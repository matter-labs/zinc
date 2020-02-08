//!
//! The semantic analyzer constant range element.
//!

use std::fmt;

use num_bigint::BigInt;

use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: BigInt,
    pub end: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Range {
    pub fn new(start: BigInt, end: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            start,
            end,
            is_signed,
            bitlength,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::range(Type::scalar(self.is_signed, self.bitlength))
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} .. {}", self.start, self.end)
    }
}
