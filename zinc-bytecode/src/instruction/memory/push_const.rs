//!
//! The 'push constant' instruction.
//!

use num_bigint::BigInt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::instruction::Instruction;
use crate::InstructionInfo;

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
}

impl InstructionInfo for Push {
    fn to_assembly(&self) -> String {
        format!("push {} as {}", self.value, self.scalar_type)
    }

    fn wrap(self) -> Instruction {
        Instruction::Push(self)
    }
}
