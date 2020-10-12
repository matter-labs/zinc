//!
//! The semantic analyzer constant range inclusive element.
//!

use std::fmt;

use num::BigInt;

use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use zinc_lexical::Location;

///
/// Inclusive range is a constant with the `start` and inclusive `end` bounds, sign, and bitlength.
///
/// Ranges are used mostly as loop bounds and the array slice operator argument.
///
#[derive(Debug, Clone, PartialEq)]
pub struct RangeInclusive {
    /// The location, where the value appears in the code.
    pub location: Location,
    /// The range start constant.
    pub start: BigInt,
    /// The range end constant.
    pub end: BigInt,
    /// If the range bounds type is signed.
    pub is_signed: bool,
    /// The bitlength, enough to fit the bigger range bound.
    pub bitlength: usize,
}

impl RangeInclusive {
    ///
    /// A shortcut constructor.
    ///
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

    ///
    /// Returns the range bound type.
    ///
    pub fn bounds_type(&self) -> Type {
        Type::scalar(Some(self.location), self.is_signed, self.bitlength)
    }
}

impl ITyped for RangeInclusive {
    fn r#type(&self) -> Type {
        Type::range_inclusive(Some(self.location), self.bounds_type())
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
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
