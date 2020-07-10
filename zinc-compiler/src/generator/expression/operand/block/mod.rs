//!
//! The generator expression block operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;
use crate::generator::statement::Statement;

///
/// The block expression which is translated to Zinc VM bytecode.
///
#[derive(Debug, Clone)]
pub struct Expression {
    statements: Vec<Statement>,
    expression: Option<GeneratorExpression>,
}

impl Expression {
    pub fn new(statements: Vec<Statement>, expression: Option<GeneratorExpression>) -> Self {
        Self {
            statements,
            expression,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        for statement in self.statements.into_iter() {
            statement.write_all_to_bytecode(bytecode.clone());
        }
        if let Some(expression) = self.expression {
            expression.write_all_to_bytecode(bytecode);
        }
    }
}
