//!
//! The generator expression block operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::statement::Statement;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

///
/// The block expression.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The block statements.
    statements: Vec<Statement>,
    /// The optional block expressions, whose type is defaulted to `()` if unset.
    expression: Option<GeneratorExpression>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(statements: Vec<Statement>, expression: Option<GeneratorExpression>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}

impl IBytecodeWritable for Expression {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        for statement in self.statements.into_iter() {
            statement.write_to_zinc_vm(state.clone());
        }
        if let Some(expression) = self.expression {
            expression.write_to_zinc_vm(state);
        }
    }
}
