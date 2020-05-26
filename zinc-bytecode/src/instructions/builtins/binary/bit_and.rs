use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitAnd;

impl InstructionInfo for BitAnd {
    fn to_assembly(&self) -> String {
        "bit_and".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitAnd(self)
    }
}
