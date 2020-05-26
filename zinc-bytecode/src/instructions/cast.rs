use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::scalar::Type as ScalarType;
use crate::Instruction;
use crate::InstructionInfo;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub r#type: ScalarType,
}

impl Cast {
    pub fn new(r#type: ScalarType) -> Self {
        Self { r#type }
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        format!("cast {}", self.r#type)
    }

    fn wrap(self) -> Instruction {
        Instruction::Cast(self)
    }
}
