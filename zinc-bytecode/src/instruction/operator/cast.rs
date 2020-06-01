//!
//! The 'cast' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cast {
    pub r#type: ScalarType,
}

impl Cast {
    pub fn new(r#type: ScalarType) -> Self {
        Self { r#type }
    }

    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::Cast(self)
    }
}

impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cast {}", self.r#type)
    }
}
