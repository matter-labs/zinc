//!
//! The generator expression memory operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::r#type::Type;
use crate::semantic::Value as SemanticValue;

#[derive(Debug, Clone)]
pub struct Memory {
    pub address: usize,
    pub r#type: Type,
}

impl Memory {
    pub fn try_from_semantic(address: usize, value: &SemanticValue) -> Option<Self> {
        Type::try_from_semantic(&value.r#type()).map(|r#type| Self { address, r#type })
    }

    pub fn write_all_to_bytecode_load(self, bytecode: Rc<RefCell<Bytecode>>) {
        bytecode.borrow_mut().push_instruction(
            Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                self.address,
                self.r#type.size(),
            )),
            crate::lexical::Location::default(),
        );
    }

    pub fn write_all_to_bytecode_store(self, bytecode: Rc<RefCell<Bytecode>>) {
        bytecode.borrow_mut().push_instruction(
            Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                self.address,
                self.r#type.size(),
            )),
            crate::lexical::Location::default(),
        );
    }
}
