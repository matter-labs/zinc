//!
//! The interpreter value.
//!

mod boolean;
mod error;
mod integer;

pub use self::boolean::Boolean;
pub use self::error::Error;
pub use self::integer::Integer;

use std::convert::TryFrom;
use std::fmt;

use num_bigint::BigInt;
use num_traits::Num;
use serde_derive::Serialize;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Value {
    Void,
    Boolean(Boolean),
    Integer(Integer),
}

impl Value {
    pub fn new_from_type(type_variant: TypeVariant) -> Self {
        match type_variant {
            TypeVariant::Void => Self::Void,
            TypeVariant::Bool => Self::Boolean(Boolean::default()),
            TypeVariant::Int { bitlength } => {
                Self::Integer(Integer::new(BigInt::default(), true, bitlength))
            }
            TypeVariant::Uint { bitlength } => {
                Self::Integer(Integer::new(BigInt::default(), false, bitlength))
            }
            TypeVariant::Field => Self::Integer(Integer::new(BigInt::default(), false, 254)),
        }
    }

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

    pub fn equal(&self, other: &Self) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(self.clone(), other.clone()));
        }

        Ok(Boolean::new(self == other))
    }

    pub fn not_equal(&self, other: &Self) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(self.clone(), other.clone()));
        }

        Ok(Boolean::new(self != other))
    }
}

impl From<BooleanLiteral> for Value {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::Boolean(Boolean::new_false()),
            BooleanLiteral::True => Self::Boolean(Boolean::new_true()),
        }
    }
}

impl TryFrom<IntegerLiteral> for Value {
    type Error = Error;

    fn try_from(integer: IntegerLiteral) -> Result<Self, Error> {
        let (value, base) = match integer {
            IntegerLiteral::Decimal { value } => (value, 10),
            IntegerLiteral::Hexadecimal { value } => (value, 16),
        };

        let value = BigInt::from_str_radix(&value, base).expect("Integer literal parsing bug");
        let mut bitlength = 8;
        let mut exponent = BigInt::from(256);
        while value >= exponent {
            if bitlength == 248 {
                exponent *= 64;
                bitlength += 6;
            } else if bitlength == 254 {
                return Err(Error::IntegerLiteralIsTooLarge(bitlength));
            } else {
                exponent *= 256;
                bitlength += 8;
            }
        }

        Ok(Self::Integer(Integer::new(value, false, bitlength)))
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
