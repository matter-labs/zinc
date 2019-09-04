//!
//! The interpreter value.
//!

mod integer;

pub use self::integer::Integer;
pub use self::integer::Type as IntegerType;

use std::fmt;
use std::str;

use num_bigint::BigInt;
use num_traits::Num;
use serde_derive::Serialize;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Value {
    Void,
    Boolean(bool),
    Integer(Integer),
}

impl Value {
    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Void, Self::Void) => true,
            (Self::Boolean(..), Self::Boolean(..)) => true,
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                integer_1.has_the_same_type_as(integer_2)
            }
            _ => false,
        }
    }
}

impl From<BooleanLiteral> for Value {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::Boolean(false),
            BooleanLiteral::True => Self::Boolean(true),
        }
    }
}

impl From<IntegerLiteral> for Value {
    fn from(integer: IntegerLiteral) -> Self {
        match integer {
            IntegerLiteral::Decimal { value } => {
                let value = BigInt::from_str_radix(unsafe { str::from_utf8_unchecked(&value) }, 10)
                    .expect("Decimal integer literal parsing bug");
                let mut bitlength = 2;
                let mut exponent = BigInt::from(4);
                while value >= exponent {
                    exponent *= 2;
                    bitlength += 1;
                }

                let r#type = if 1 <= bitlength && bitlength <= 253 {
                    IntegerType::Uint { bitlength }
                } else if bitlength == 254 {
                    IntegerType::Field
                } else {
                    unreachable!();
                };

                Self::Integer(Integer::new(value, r#type))
            }
            IntegerLiteral::Hexadecimal { value } => {
                let bitlength = value.len() * 4;

                let value = BigInt::from_str_radix(unsafe { str::from_utf8_unchecked(&value) }, 16)
                    .expect("Hexadecimal integer literal parsing bug");
                let r#type = if 1 <= bitlength && bitlength <= 253 {
                    IntegerType::Uint { bitlength }
                } else if bitlength == 254 {
                    IntegerType::Field
                } else {
                    unreachable!();
                };

                Self::Integer(Integer::new(value, r#type))
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "(): ()"),
            Self::Boolean(boolean) => write!(f, "{}: bool", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}
