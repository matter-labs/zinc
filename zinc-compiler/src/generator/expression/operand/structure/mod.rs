//!
//! The generator expression structure operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode::Bytecode;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;

#[derive(Debug, Clone)]
pub struct Expression {
    fields: Vec<(String, Type, GeneratorExpression)>,
}

impl Expression {
    pub fn new(fields: Vec<(String, Type, GeneratorExpression)>) -> Self {
        Self { fields }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        for (_name, _type, expression) in self.fields.into_iter() {
            expression.write_all_to_bytecode(bytecode.clone());
        }
    }
}
