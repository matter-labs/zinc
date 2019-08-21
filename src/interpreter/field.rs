//!
//! The interpreter field.
//!

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Serialize;

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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.value_type)
    }
}
