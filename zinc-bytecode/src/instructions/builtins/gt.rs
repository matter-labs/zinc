use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Gt;

impl InstructionInfo for Gt {
    fn to_assembly(&self) -> String {
        "gt".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Gt((*self).clone())
    }
}
