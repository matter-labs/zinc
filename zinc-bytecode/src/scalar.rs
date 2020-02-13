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
    pub signed: bool,
    pub length: usize,
}

impl ScalarType {
    pub fn is_signed(&self) -> bool {
        match self {
            ScalarType::Integer(IntegerType { signed: true, .. }) => true,
            _ => false,
        }
    }
}

impl IntegerType {
    pub const BOOLEAN: Self = IntegerType {
        signed: false,
        length: 1,
    };

    pub const U8: Self = IntegerType {
        signed: false,
        length: 8,
    };
    pub const I8: Self = IntegerType {
        signed: true,
        length: 8,
    };

    pub const U16: Self = IntegerType {
        signed: false,
        length: 16,
    };
    pub const I16: Self = IntegerType {
        signed: true,
        length: 16,
    };

    // Add more if needed for convenience...
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
                if int_type.signed { "i" } else { "u" },
                int_type.length,
            ),
        }
    }
}
