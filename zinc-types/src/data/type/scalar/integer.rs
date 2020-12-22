//!
//! The integer scalar type.
//!

use std::fmt;

use num::BigInt;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;

///
/// The scalar integer type.
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
    pub const U1: Self = Self {
        is_signed: false,
        bitlength: zinc_const::bitlength::BOOLEAN,
    };

    /// An auxiliary internal type.
    pub const U8: Self = Self {
        is_signed: false,
        bitlength: zinc_const::bitlength::BYTE,
    };

    /// An auxiliary internal type.
    pub const I8: Self = Self {
        is_signed: true,
        bitlength: zinc_const::bitlength::BYTE,
    };

    /// An auxiliary internal type.
    pub const U16: Self = Self {
        is_signed: false,
        bitlength: zinc_const::bitlength::BYTE * 2,
    };

    /// An auxiliary internal type.
    pub const I16: Self = Self {
        is_signed: true,
        bitlength: zinc_const::bitlength::BYTE * 2,
    };

    /// An auxiliary internal type.
    pub const ETH_ADDRESS: Self = Self {
        is_signed: false,
        bitlength: zinc_const::bitlength::ETH_ADDRESS,
    };

    /// An auxiliary internal type.
    pub const BALANCE: Self = Self {
        is_signed: false,
        bitlength: zinc_const::bitlength::BALANCE,
    };

    ///
    /// A shortcut constructor.
    ///
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            is_signed,
            bitlength,
        }
    }

    ///
    /// Returns the minimum value of the type.
    ///
    pub fn min(&self) -> BigInt {
        if self.is_signed {
            -(BigInt::from(1) << self.bitlength) / 2
        } else {
            BigInt::zero()
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
