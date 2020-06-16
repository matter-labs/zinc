//!
//! The 'debug' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dbg {
    pub format: String,
    pub argument_types: Vec<DataType>,
}

impl Dbg {
    pub fn new(format: String, arg_types: Vec<DataType>) -> Self {
        Self {
            format,
            argument_types: arg_types,
        }
    }

    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Dbg {
    fn into(self) -> Instruction {
        Instruction::Dbg(self)
    }
}

impl fmt::Display for Dbg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dbg")
    }
}
