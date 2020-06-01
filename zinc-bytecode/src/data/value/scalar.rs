//!
//! The Zinc VM template scalar value.
//!

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::scalar::integer::Type as IntegerType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Integer(BigInt, IntegerType),
    Field(BigInt),
}

impl Value {
    pub fn to_bigint(&self) -> BigInt {
        match self {
            Self::Bool(value) => {
                if *value {
                    BigInt::one()
                } else {
                    BigInt::zero()
                }
            }
            Self::Field(value) | Self::Integer(value, _) => value.clone(),
        }
    }
}
