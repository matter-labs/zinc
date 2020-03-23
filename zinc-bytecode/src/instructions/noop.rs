use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct NoOperation;

impl InstructionInfo for NoOperation {
    fn to_assembly(&self) -> String {
        "noop".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::NoOperation((*self).clone())
    }
}
