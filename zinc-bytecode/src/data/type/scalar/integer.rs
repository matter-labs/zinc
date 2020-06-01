//!
//! The Zinc VM template integer scalar type.
//!

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Type {
    /// Auxiliary internal type
    pub const U1: Self = Type {
        is_signed: false,
        bitlength: 1,
    };

    pub const U8: Self = Type {
        is_signed: false,
        bitlength: 8,
    };

    pub const I8: Self = Type {
        is_signed: true,
        bitlength: 8,
    };

    pub const U16: Self = Type {
        is_signed: false,
        bitlength: 16,
    };

    pub const I16: Self = Type {
        is_signed: true,
        bitlength: 16,
    };

    // Add more if needed for convenience...

    pub fn min(&self) -> BigInt {
        if self.is_signed {
            -(BigInt::from(1) << self.bitlength) / 2
        } else {
            0.into()
        }
    }

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
