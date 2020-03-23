use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Swap;

impl InstructionInfo for Swap {
    fn to_assembly(&self) -> String {
        "swap".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Swap((*self).clone())
    }
}
