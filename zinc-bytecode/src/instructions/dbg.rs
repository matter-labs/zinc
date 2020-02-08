use crate::data::types::DataType;
use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Dbg {
    pub format: String,
    pub arg_types: Vec<DataType>,
}

impl Dbg {
    pub fn new(format: String, arg_types: Vec<DataType>) -> Self {
        Self { format, arg_types }
    }
}

impl InstructionInfo for Dbg {
    fn to_assembly(&self) -> String {
        "dbg".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Dbg
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Dbg((*self).clone())
    }
}
