//!
//! The semantic analyzer constant range inclusive element.
//!

use std::fmt;

use num_bigint::BigInt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// Inclusive range is a constant with the `start` and inclusive `end` bounds, sign, and bitlength.
///
/// Ranges are used mostly as loop bounds and the array slice operator argument.
///
#[derive(Debug, Clone, PartialEq)]
pub struct RangeInclusive {
    pub location: Location,
    pub start: BigInt,
    pub end: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl RangeInclusive {
    pub fn new(
        location: Location,
        start: BigInt,
        end: BigInt,
        is_signed: bool,
        bitlength: usize,
    ) -> Self {
        Self {
            location,
            start,
            end,
            is_signed,
            bitlength,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::range_inclusive(Some(self.location), self.bounds_type())
    }

    pub fn bounds_type(&self) -> Type {
        Type::scalar(Some(self.location), self.is_signed, self.bitlength)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for RangeInclusive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{} ..= {}' of type '{}'",
            self.start,
            self.end,
            self.bounds_type()
        )
    }
}
