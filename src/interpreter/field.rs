//!
//! The interpreter field.
//!

use std::fmt;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::lexical::Literal;
use crate::syntax::Type;

#[derive(Debug, Serialize)]
pub struct Field {
    #[serde(skip_serializing)]
    pub value: BigInt,
    pub value_type: Type,
}

impl Field {
    pub fn new(value: BigInt, value_type: Type) -> Self {
        Self { value, value_type }
    }
}

impl From<Literal> for Field {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Boolean(boolean) => Self::from(boolean),
            Literal::Integer(integer) => Self::from(integer),
        }
    }
}

impl From<BooleanLiteral> for Field {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::new(BigInt::zero(), Type::Bool),
            BooleanLiteral::True => Self::new(BigInt::one(), Type::Bool),
        }
    }
}

impl From<IntegerLiteral> for Field {
    fn from(integer: IntegerLiteral) -> Self {
        match integer {
            IntegerLiteral::Decimal { value } => {
                let value = BigInt::from_str_radix(value.as_str(), 10)
                    .expect("Decimal integer literal parsing bug");
                Self::new(value, Type::Field)
            }
            IntegerLiteral::Hexadecimal { value } => {
                let value = BigInt::from_str_radix(value.as_str(), 16)
                    .expect("Hexadecimal integer literal parsing bug");
                Self::new(value, Type::Field)
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.value_type)
    }
}
