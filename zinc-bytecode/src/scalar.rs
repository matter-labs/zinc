use num_bigint::BigInt;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ScalarType {
    Field,
    Boolean,
    Integer(IntegerType),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IntegerType {
    pub is_signed: bool,
    pub bitlength: usize,
}

impl ScalarType {
    pub fn is_signed(&self) -> bool {
        match self {
            ScalarType::Integer(IntegerType {
                is_signed: true, ..
            }) => true,
            _ => false,
        }
    }
}

impl IntegerType {
    /// Auxiliary internal type
    pub const U1: Self = IntegerType {
        is_signed: false,
        bitlength: 1,
    };

    pub const U8: Self = IntegerType {
        is_signed: false,
        bitlength: 8,
    };

    pub const I8: Self = IntegerType {
        is_signed: true,
        bitlength: 8,
    };

    pub const U16: Self = IntegerType {
        is_signed: false,
        bitlength: 16,
    };

    pub const I16: Self = IntegerType {
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

impl From<IntegerType> for ScalarType {
    fn from(int_type: IntegerType) -> Self {
        ScalarType::Integer(int_type)
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Self::Field => write!(f, "field"),
            Self::Boolean => write!(f, "bool"),
            Self::Integer(int_type) => write!(
                f,
                "{}{}",
                if int_type.is_signed { "i" } else { "u" },
                int_type.bitlength,
            ),
        }
    }
}
