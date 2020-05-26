use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitShiftLeft;

impl InstructionInfo for BitShiftLeft {
    fn to_assembly(&self) -> String {
        "bit_shift_left".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitShiftLeft(self)
    }
}
