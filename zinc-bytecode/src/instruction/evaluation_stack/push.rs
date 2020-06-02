//!
//! The 'push constant' instruction.
//!

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Push {
    pub value: BigInt,
    pub scalar_type: ScalarType,
}

impl Push {
    pub fn new(value: BigInt, scalar_type: ScalarType) -> Self {
        Self { value, scalar_type }
    }

    pub fn new_field(value: BigInt) -> Self {
        Self::new(value, ScalarType::Field)
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Push {
    fn into(self) -> Instruction {
        Instruction::Push(self)
    }
}

impl fmt::Display for Push {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "push {} as {}", self.value, self.scalar_type)
    }
}
