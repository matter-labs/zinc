use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct BitXor;

impl InstructionInfo for BitXor {
    fn to_assembly(&self) -> String {
        "bit_xor".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::BitXor
    }

    fn wrap(&self) -> Instruction {
        Instruction::BitXor(self.clone())
    }
}
