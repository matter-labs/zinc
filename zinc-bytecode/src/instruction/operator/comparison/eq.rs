//!
//! The 'equals comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Eq;

impl InstructionInfo for Eq {
    fn to_assembly(&self) -> String {
        "eq".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Eq(self)
    }
}
