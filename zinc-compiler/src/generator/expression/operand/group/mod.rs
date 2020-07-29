//!
//! The generator expression group operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;

///
/// The group expression which is translated to Zinc VM data.
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
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        for (_type, expression) in self.expressions.into_iter() {
            expression.write_all(bytecode.clone());
        }
    }
}
