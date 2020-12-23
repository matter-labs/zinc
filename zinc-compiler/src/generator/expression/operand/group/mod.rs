//!
//! The generator expression group operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

///
/// The group expression which is translated to some data.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The typed group element expressions.
    expressions: Vec<(Type, GeneratorExpression)>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(expressions: Vec<(Type, GeneratorExpression)>) -> Self {
        Self { expressions }
    }
}

impl IBytecodeWritable for Expression {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        for (_type, expression) in self.expressions.into_iter() {
            expression.write_to_zinc_vm(state.clone());
        }
    }
}
