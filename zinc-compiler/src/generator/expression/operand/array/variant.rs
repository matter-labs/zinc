//!
//! The generator expression array operand variant.
//!

use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Variant {
    List {
        expressions: Vec<GeneratorExpression>,
    },
    Repeated {
        expression: GeneratorExpression,
        size: usize,
    },
}

impl Variant {
    pub fn new_list(expressions: Vec<GeneratorExpression>) -> Self {
        Self::List { expressions }
    }

    pub fn new_repeated(expression: GeneratorExpression, size: usize) -> Self {
        Self::Repeated { expression, size }
    }
}
