use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct And;

impl InstructionInfo for And {
    fn to_assembly(&self) -> String {
        "and".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::And((*self).clone())
    }
}
