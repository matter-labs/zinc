use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitXor;

impl InstructionInfo for BitXor {
    fn to_assembly(&self) -> String {
        "bit_xor".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitXor(self)
    }
}
