use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Copies the top element from the stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tee;

impl InstructionInfo for Tee {
    fn to_assembly(&self) -> String {
        "tee".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::Tee((*self).clone())
    }
}
