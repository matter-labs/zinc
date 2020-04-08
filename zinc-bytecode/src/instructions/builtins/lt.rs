use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Lt;

impl InstructionInfo for Lt {
    fn to_assembly(&self) -> String {
        "lt".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Lt((*self).clone())
    }
}
