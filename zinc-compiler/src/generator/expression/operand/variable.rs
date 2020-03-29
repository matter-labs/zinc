//!
//! The generator expression variable operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::r#type::Type;
use crate::semantic::Value as SemanticValue;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub r#type: Type,
}

impl Variable {
    pub fn try_from_semantic(name: String, value: &SemanticValue) -> Option<Self> {
        Type::try_from_semantic(&value.r#type()).map(|r#type| Self { name, r#type })
    }

    pub fn write_all_to_bytecode_load(self, bytecode: Rc<RefCell<Bytecode>>) {
        let address = bytecode
            .borrow()
            .get_variable_address(self.name.as_str())
            .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS);

        bytecode.borrow_mut().push_instruction(
            Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                address,
                self.r#type.size(),
            )),
            crate::lexical::Location::default(),
        );
    }

    pub fn write_all_to_bytecode_store(self, bytecode: Rc<RefCell<Bytecode>>) {
        let address = bytecode
            .borrow()
            .get_variable_address(self.name.as_str())
            .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS);

        bytecode.borrow_mut().push_instruction(
            Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                address,
                self.r#type.size(),
            )),
            crate::lexical::Location::default(),
        );
    }
}
