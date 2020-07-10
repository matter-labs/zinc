//!
//! The generator expression array operand.
//!

pub mod builder;
pub mod variant;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;

use self::variant::Variant;

///
/// The array expression which is translated to Zinc VM data.
///
#[derive(Debug, Clone)]
pub struct Expression {
    variant: Variant,
}

impl Expression {
    pub fn new_list(expressions: Vec<GeneratorExpression>) -> Self {
        Self {
            variant: Variant::new_list(expressions),
        }
    }

    pub fn new_repeated(expression: GeneratorExpression, size: usize) -> Self {
        Self {
            variant: Variant::new_repeated(expression, size),
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        match self.variant {
            Variant::List { expressions } => {
                for expression in expressions.into_iter() {
                    expression.write_all_to_bytecode(bytecode.clone());
                }
            }
            Variant::Repeated { expression, size } => {
                for expression in vec![expression; size].into_iter() {
                    expression.write_all_to_bytecode(bytecode.clone());
                }
            }
        }
    }
}
