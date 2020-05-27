//!
//! The 'logical OR' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Or;

impl InstructionInfo for Or {
    fn to_assembly(&self) -> String {
        "or".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Or(self)
    }
}
