//!
//! The 'not equals comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ne;

impl InstructionInfo for Ne {
    fn to_assembly(&self) -> String {
        "ne".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Ne(self)
    }
}
