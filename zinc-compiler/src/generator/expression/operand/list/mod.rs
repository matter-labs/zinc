//!
//! The generator expression list operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;

///
/// The list expression which is translated to Zinc VM data.
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
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        for expression in self.expressions.into_iter() {
            expression.write_all(bytecode.clone());
        }
    }
}
