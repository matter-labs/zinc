//!
//! The generator expression variable operand.
//!

use zinc_bytecode::Instruction;

use crate::generator::r#type::Type;
use crate::semantic::Value as SemanticValue;

#[derive(Debug, Clone)]
pub struct Variable {
    pub r#type: Type,
}

impl Variable {
    pub fn try_from_semantic(value: &SemanticValue) -> Option<Self> {
        Type::try_from_semantic(&value.r#type()).map(|r#type| Self { r#type })
    }

    pub fn into_instruction_load(self) -> Instruction {
        Instruction::Load(zinc_bytecode::Load::new(0 /* TODO */))
    }

    pub fn into_instruction_store(self) -> Instruction {
        Instruction::Store(zinc_bytecode::Store::new(0 /* TODO */))
    }
}
