use std::fmt;
use crate::RuntimeError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarType {
    Field,
    Integer(IntegerType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegerType {
    pub signed: bool,
    pub length: usize,
}

impl IntegerType {
    pub const BOOLEAN: Self = IntegerType {
        signed: false,
        length: 1,
    };
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
            Self::Integer(int_type) => {
                if int_type == &IntegerType::BOOLEAN {
                    write!(f, "bool")
                } else {
                    write!(f, "{}{}", if int_type.signed { "i" } else { "u" }, int_type.length,)
                }
            }
        }
    }
}

impl ScalarType {
    pub fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError> {
        if left == right {
            Ok(left)
        } else {
            Err(RuntimeError::TypeError {
                expected: left.to_string(),
                actual: right.to_string(),
            })
        }
    }
}
