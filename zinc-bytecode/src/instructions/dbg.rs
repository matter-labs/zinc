use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    fn wrap(self) -> Instruction {
        Instruction::Dbg(self)
    }
}
