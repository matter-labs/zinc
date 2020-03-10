use crate::builtins::BuiltinIdentifier;
use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    fn code() -> InstructionCode {
        InstructionCode::CallBuiltin
    }

    fn inputs_count(&self) -> usize {
        self.inputs_count
    }

    fn outputs_count(&self) -> usize {
        self.outputs_count
    }

    fn wrap(&self) -> Instruction {
        Instruction::CallBuiltin((*self).clone())
    }
}
