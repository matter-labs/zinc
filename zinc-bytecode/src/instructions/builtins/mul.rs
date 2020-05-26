use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Mul;

impl InstructionInfo for Mul {
    fn to_assembly(&self) -> String {
        "mul".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Mul(self)
    }
}
