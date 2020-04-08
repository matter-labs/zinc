use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct LoopEnd;

impl InstructionInfo for LoopEnd {
    fn to_assembly(&self) -> String {
        "loop_end".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoopEnd((*self).clone())
    }
}
