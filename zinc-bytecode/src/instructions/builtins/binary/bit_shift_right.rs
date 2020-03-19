use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct BitShiftRight;

impl InstructionInfo for BitShiftRight {
    fn to_assembly(&self) -> String {
        "bit_shift_right".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::BitShiftRight
    }

    fn wrap(&self) -> Instruction {
        Instruction::BitShiftRight(self.clone())
    }
}
