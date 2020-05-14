//!
//! The intermediate representation for Zinc VM bytecode generating.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;
use crate::generator::statement::Statement;

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        for statement in self.statements.into_iter() {
            statement.write_all_to_bytecode(bytecode.clone());
        }
    }
}
