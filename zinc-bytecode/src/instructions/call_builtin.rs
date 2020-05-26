use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::builtin::BuiltinIdentifier;
use crate::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallBuiltin {
    pub identifier: BuiltinIdentifier,
    pub inputs_count: usize,
    pub outputs_count: usize,
}

impl CallBuiltin {
    pub fn new(identifier: BuiltinIdentifier, inputs_count: usize, outputs_count: usize) -> Self {
        Self {
            identifier,
            inputs_count,
            outputs_count,
        }
    }
}

impl InstructionInfo for CallBuiltin {
    fn to_assembly(&self) -> String {
        format!(
            "call_builtin {:?}({}) -> {}",
            self.identifier, self.inputs_count, self.outputs_count
        )
    }

    fn wrap(self) -> Instruction {
        Instruction::CallBuiltin(self)
    }
}
