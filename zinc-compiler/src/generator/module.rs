//!
//! The intermediate representation for a Zinc module.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::state::State;
use crate::generator::statement::Statement;

#[derive(Default)]
pub struct Module {
    pub statements: Vec<Statement>,
}

impl Module {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        for statement in self.statements.into_iter() {
            statement.write_all_to_bytecode(bytecode.clone());
        }
    }
}
