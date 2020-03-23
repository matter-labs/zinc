//!
//! The generator expression array operand.
//!

pub mod builder;
pub mod variant;

use crate::generator::expression::Expression as GeneratorExpression;

use self::variant::Variant;

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
}
