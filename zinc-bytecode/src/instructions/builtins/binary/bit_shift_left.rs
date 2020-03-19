use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct BitShiftLeft;

impl InstructionInfo for BitShiftLeft {
    fn to_assembly(&self) -> String {
        "bit_shift_left".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::BitShiftLeft
    }

    fn wrap(&self) -> Instruction {
        Instruction::BitShiftLeft(self.clone())
    }
}
