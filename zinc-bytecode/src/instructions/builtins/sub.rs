use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Sub;

impl InstructionInfo for Sub {
    fn to_assembly(&self) -> String {
        "sub".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Sub((*self).clone())
    }
}
