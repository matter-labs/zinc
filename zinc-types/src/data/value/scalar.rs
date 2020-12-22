//!
//! The template scalar value.
//!

use num::BigInt;
use num::One;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::scalar::integer::Type as IntegerType;

///
/// The template scalar value.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// Represented with JSON boolean.
    Boolean(bool),
    /// Represented with numeric string. We cannot use the JSON native numeric type, because
    /// it cannot store large values like `2^253`.
    Integer(BigInt, IntegerType),
    /// Represented with numeric string. We cannot use the JSON native numeric type, because
    /// it cannot store large values like `2^253`.
    Field(BigInt),
}

impl Value {
    ///
    /// Converts the value to a `BigInt`.
    ///
    pub fn to_bigint(&self) -> BigInt {
        match self {
            Self::Boolean(value) => {
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
