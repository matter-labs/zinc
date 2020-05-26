use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitNot;

impl InstructionInfo for BitNot {
    fn to_assembly(&self) -> String {
        "bit_not".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitNot(self)
    }
}
