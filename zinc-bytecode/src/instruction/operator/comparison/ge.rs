//!
//! The 'greater or equal comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ge;

impl InstructionInfo for Ge {
    fn to_assembly(&self) -> String {
        "ge".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Ge(self)
    }
}
