//!
//! The 'arithmetic division' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Div;

impl InstructionInfo for Div {
    fn to_assembly(&self) -> String {
        "div".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Div(self)
    }
}
