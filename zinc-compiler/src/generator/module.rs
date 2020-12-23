//!
//! The intermediate representation for a Zinc module.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement;
use crate::generator::zinc_vm::State as ZincVMState;
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
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        for statement in self.statements.into_iter() {
            statement.write_to_zinc_vm(state.clone());
        }
    }
}
