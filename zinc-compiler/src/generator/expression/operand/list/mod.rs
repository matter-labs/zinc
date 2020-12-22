//!
//! The generator expression list operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

///
/// The list expression which is translated to some data.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The function argument expression array.
    expressions: Vec<GeneratorExpression>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(expressions: Vec<GeneratorExpression>) -> Self {
        Self { expressions }
    }
}

impl IBytecodeWritable for Expression {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        for expression in self.expressions.into_iter() {
            expression.write_to_zinc_vm(state.clone());
        }
    }
}
