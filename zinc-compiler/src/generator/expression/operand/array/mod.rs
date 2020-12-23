//!
//! The generator expression array operand.
//!

pub mod builder;
pub mod variant;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

use self::variant::Variant;

///
/// The array expression which is translated to some data.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The array expression variant.
    variant: Variant,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_list(expressions: Vec<GeneratorExpression>) -> Self {
        Self {
            variant: Variant::new_list(expressions),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_repeated(expression: GeneratorExpression, size: usize) -> Self {
        Self {
            variant: Variant::new_repeated(expression, size),
        }
    }
}

impl IBytecodeWritable for Expression {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        match self.variant {
            Variant::List { expressions } => {
                for expression in expressions.into_iter() {
                    expression.write_to_zinc_vm(state.clone());
                }
            }
            Variant::Repeated { expression, size } => {
                for expression in vec![expression; size].into_iter() {
                    expression.write_to_zinc_vm(state.clone());
                }
            }
        }
    }
}
