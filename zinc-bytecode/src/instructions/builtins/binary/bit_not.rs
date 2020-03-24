use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct BitNot;

impl InstructionInfo for BitNot {
    fn to_assembly(&self) -> String {
        "bit_shift_left".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::BitNot(self.clone())
    }
}
