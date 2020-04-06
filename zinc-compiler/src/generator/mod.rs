//!
//! The intermediate representation for Zinc VM bytecode generating.
//!

pub mod bytecode;
pub mod expression;
pub mod statement;
pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use self::bytecode::Bytecode;
use self::statement::Statement;

#[derive(Default)]
pub struct Tree {
    pub statements: Vec<Statement>,
}

impl Tree {
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
