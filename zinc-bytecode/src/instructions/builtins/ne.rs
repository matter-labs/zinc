use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Ne;

impl InstructionInfo for Ne {
    fn to_assembly(&self) -> String {
        "ne".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Ne((*self).clone())
    }
}
