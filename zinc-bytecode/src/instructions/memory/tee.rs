use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

/// Copies the top element from the stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tee;

impl InstructionInfo for Tee {
    fn to_assembly(&self) -> String {
        "tee".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Tee
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        2
    }

    fn wrap(&self) -> Instruction {
        Instruction::Tee((*self).clone())
    }
}
