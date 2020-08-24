//!
//! The Zinc VM integer scalar type.
//!

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

///
/// The Zinc VM scalar integer type.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    /// If the integer type is signed.
    pub is_signed: bool,
    /// The bitlength of the integer type.
    pub bitlength: usize,
}

impl Type {
    /// An auxiliary internal type.
    pub const U1: Self = Type {
        is_signed: false,
        bitlength: 1,
    };

    /// An auxiliary internal type.
    pub const U8: Self = Type {
        is_signed: false,
        bitlength: 8,
    };

    /// An auxiliary internal type.
    pub const I8: Self = Type {
        is_signed: true,
        bitlength: 8,
    };

    /// An auxiliary internal type.
    pub const U16: Self = Type {
        is_signed: false,
        bitlength: 16,
    };

    /// An auxiliary internal type.
    pub const I16: Self = Type {
        is_signed: true,
        bitlength: 16,
    };

    ///
    /// Returns the minimum value of the type.
    ///
    pub fn min(&self) -> BigInt {
        if self.is_signed {
            -(BigInt::from(1) << self.bitlength) / 2
        } else {
            0.into()
        }
    }

    ///
    /// Returns the maximum value of the type.
    ///
    pub fn max(&self) -> BigInt {
        if self.is_signed {
            (BigInt::from(1) << self.bitlength) / 2 - 1u8
        } else {
            (BigInt::from(1) << self.bitlength) - 1u8
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_signed { "i" } else { "u" },
            self.bitlength,
        )
    }
}
