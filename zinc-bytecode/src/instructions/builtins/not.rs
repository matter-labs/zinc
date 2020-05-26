use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Not;

impl InstructionInfo for Not {
    fn to_assembly(&self) -> String {
        "not".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Not(self)
    }
}
