//!
//! The 'arithmetic negation' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Neg;

impl InstructionInfo for Neg {
    fn to_assembly(&self) -> String {
        "neg".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Neg(self)
    }
}
