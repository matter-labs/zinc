use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Neg;

impl InstructionInfo for Neg {
    fn to_assembly(&self) -> String {
        "neg".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Neg(self)
    }
}
