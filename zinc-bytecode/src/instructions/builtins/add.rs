use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Add;

impl InstructionInfo for Add {
    fn to_assembly(&self) -> String {
        "add".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Add((*self).clone())
    }
}
