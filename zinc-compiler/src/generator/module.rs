//!
//! The intermediate representation for a Zinc module.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::state::State;
use crate::generator::statement::Statement;
use crate::generator::IBytecodeWritable;

///
/// The Zinc module, which is located in a separate file and consists of module-level statements.
///
#[derive(Default)]
pub struct Module {
    /// The inner statements array.
    pub statements: Vec<Statement>,
}

impl Module {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

impl IBytecodeWritable for Module {
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        for statement in self.statements.into_iter() {
            statement.write_all(bytecode.clone());
        }
    }
}
