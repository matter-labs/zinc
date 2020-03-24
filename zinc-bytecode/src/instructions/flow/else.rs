use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Else;

impl InstructionInfo for Else {
    fn to_assembly(&self) -> String {
        "else".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Else((*self).clone())
    }
}
