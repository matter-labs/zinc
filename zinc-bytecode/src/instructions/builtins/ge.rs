use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ge;

impl InstructionInfo for Ge {
    fn to_assembly(&self) -> String {
        "ge".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Ge(self)
    }
}
