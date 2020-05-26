use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Instruction;
use crate::InstructionInfo;

/// Copies the top element from the stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Copy;

impl InstructionInfo for Copy {
    fn to_assembly(&self) -> String {
        "copy".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Copy(self)
    }
}
