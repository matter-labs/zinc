//!
//! The `push constant` instruction.
//!

use std::fmt;

use num::BigInt;
use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::instructions::Instruction;

///
/// The `push constant` instruction.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Push {
    /// The constant value.
    pub value: BigInt,
    /// The constant type.
    pub scalar_type: ScalarType,
}

impl Push {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(value: BigInt, scalar_type: ScalarType) -> Self {
        Self { value, scalar_type }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_field(value: BigInt) -> Self {
        Self::new(value, ScalarType::Field)
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
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
