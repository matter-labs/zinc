//!
//! The 'logical AND' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct And;

impl InstructionInfo for And {
    fn to_assembly(&self) -> String {
        "and".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::And(self)
    }
}
