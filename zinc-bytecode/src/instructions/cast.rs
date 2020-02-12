use crate::scalar::{IntegerType, ScalarType};
use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub scalar_type: ScalarType,
}

impl Cast {
    pub fn new(scalar_type: ScalarType) -> Self {
        Self { scalar_type }
    }

    #[deprecated(note = "this is temporary fix")]
    pub fn new_integer(signed: bool, length: usize) -> Self {
        Self::new(IntegerType { signed, length }.into())
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        format!("cast {}", self.scalar_type)
    }

    fn code() -> InstructionCode {
        InstructionCode::Cast
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Cast((*self).clone())
    }
}
