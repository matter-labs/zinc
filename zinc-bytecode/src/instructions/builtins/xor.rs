use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Xor;

impl InstructionInfo for Xor {
    fn to_assembly(&self) -> String {
        "xor".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Xor((*self).clone())
    }
}
