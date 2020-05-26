use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct If;

impl InstructionInfo for If {
    fn to_assembly(&self) -> String {
        "if".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::If(self)
    }
}
