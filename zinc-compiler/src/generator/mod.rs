//!
//! The generator intermediate language.
//!

pub mod expression;
pub mod statement;
pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode::Bytecode;

use self::statement::Statement;

pub static PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS: &str =
    "Validated during the semantic analysis";

#[derive(Default)]
pub struct Representation {
    pub statements: Vec<Statement>,
}

impl Representation {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        for statement in self.statements.into_iter() {
            statement.write_all_to_bytecode(bytecode.clone());
        }
    }
}
