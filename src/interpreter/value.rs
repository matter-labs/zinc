//!
//! The interpreter value.
//!

use std::fmt;
use std::str;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Value {
    #[serde(skip_serializing)]
    pub field: BigInt,
    pub type_variant: TypeVariant,
}

impl Value {
    pub fn new(field: BigInt, type_variant: TypeVariant) -> Self {
        Self {
            field,
            type_variant,
        }
    }

    pub fn cast(self, type_variant: TypeVariant) -> Result<Self, OperatorError> {
        match (self.type_variant, type_variant) {
            (TypeVariant::Uint { bitlength: b1 }, TypeVariant::Uint { bitlength: b2 })
                if b1 > b2 =>
            {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.type_variant,
                    type_variant,
                ));
            }
            (TypeVariant::Int { bitlength: b1 }, TypeVariant::Int { bitlength: b2 }) if b1 > b2 => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.type_variant,
                    type_variant,
                ));
            }
            (TypeVariant::Uint { bitlength: b1 }, TypeVariant::Int { bitlength: b2 })
                if b1 >= b2 =>
            {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.type_variant,
                    type_variant,
                ));
            }
            (TypeVariant::Int { bitlength: b1 }, TypeVariant::Uint { bitlength: b2 }) => {
                if b1 >= b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.type_variant,
                        type_variant,
                    ));
                }
            }
            (TypeVariant::Uint { .. }, TypeVariant::Field) => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.type_variant,
                    type_variant,
                ));
            }
            (TypeVariant::Int { .. }, TypeVariant::Field) => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.type_variant,
                    type_variant,
                ));
            }
            _ => {
                return Err(OperatorError::casting_invalid_types(
                    self.type_variant,
                    type_variant,
                ));
            }
        }

        Ok(Self::new(self.field, type_variant))
    }
}

impl From<BooleanLiteral> for Value {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::new(BigInt::zero(), TypeVariant::Bool),
            BooleanLiteral::True => Self::new(BigInt::one(), TypeVariant::Bool),
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

                let type_variant = if 2 <= bitlength && bitlength <= 253 {
                    TypeVariant::Uint { bitlength }
                } else if bitlength == 254 {
                    TypeVariant::Field
                } else {
                    unreachable!();
                };

                Self::new(value, type_variant)
            }
            IntegerLiteral::Hexadecimal { value } => {
                let bitlength = value.len() * 4;

                let value = BigInt::from_str_radix(unsafe { str::from_utf8_unchecked(&value) }, 16)
                    .expect("Hexadecimal integer literal parsing bug");
                let type_variant = if 2 <= bitlength && bitlength <= 253 {
                    TypeVariant::Uint { bitlength }
                } else if bitlength == 254 {
                    TypeVariant::Field
                } else {
                    unreachable!();
                };

                Self::new(value, type_variant)
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.type_variant)
    }
}
