use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Le;

impl InstructionInfo for Le {
    fn to_assembly(&self) -> String {
        "le".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Le(self)
    }
}
