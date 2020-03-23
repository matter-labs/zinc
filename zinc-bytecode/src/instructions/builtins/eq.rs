use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Eq;

impl InstructionInfo for Eq {
    fn to_assembly(&self) -> String {
        "eq".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Eq((*self).clone())
    }
}
