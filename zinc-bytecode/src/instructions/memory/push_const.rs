use crate::scalar::ScalarType;
use crate::{Instruction, InstructionCode, InstructionInfo};
use num_bigint::BigInt;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PushConst {
    pub value: BigInt,
    pub scalar_type: ScalarType,
}

impl PushConst {
    pub fn new(value: BigInt, scalar_type: ScalarType) -> Self {
        Self { value, scalar_type }
    }

    pub fn new_untyped(value: BigInt) -> Self {
        Self::new(value, ScalarType::Field)
    }
}

impl InstructionInfo for PushConst {
    fn to_assembly(&self) -> String {
        format!("push {} as {}", self.value, self.scalar_type)
    }

    fn code() -> InstructionCode {
        InstructionCode::PushConst
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::PushConst((*self).clone())
    }
}
