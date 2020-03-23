use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Or;

impl InstructionInfo for Or {
    fn to_assembly(&self) -> String {
        "or".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Or((*self).clone())
    }
}
