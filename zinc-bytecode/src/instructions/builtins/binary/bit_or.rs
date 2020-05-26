use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitOr;

impl InstructionInfo for BitOr {
    fn to_assembly(&self) -> String {
        "bit_or".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitOr(self)
    }
}
