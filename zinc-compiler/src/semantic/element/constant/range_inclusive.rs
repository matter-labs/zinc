//!
//! The semantic analyzer constant range inclusive element.
//!

use std::fmt;

use num_bigint::BigInt;

use crate::semantic::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct RangeInclusive {
    pub start: BigInt,
    pub end: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl RangeInclusive {
    pub fn new(start: BigInt, end: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            start,
            end,
            is_signed,
            bitlength,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::new_range(Type::new_numeric(self.is_signed, self.bitlength))
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for RangeInclusive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ..= {}", self.start, self.end)
    }
}
