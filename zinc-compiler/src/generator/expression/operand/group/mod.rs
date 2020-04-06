//!
//! The generator expression group operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;

///
/// The group expression which is translated to Zinc VM data.
///
#[derive(Debug, Clone)]
pub struct Expression {
    expressions: Vec<(Type, GeneratorExpression)>,
}

impl Expression {
    pub fn new(expressions: Vec<(Type, GeneratorExpression)>) -> Self {
        Self { expressions }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        for (_type, expression) in self.expressions.into_iter() {
            expression.write_all_to_bytecode(bytecode.clone());
        }
    }
}
