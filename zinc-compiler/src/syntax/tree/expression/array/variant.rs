//!
//! The array expression variant.
//!

use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    List {
        elements: Vec<Expression>,
    },
    Repeated {
        expression: Expression,
        size_expression: Expression,
    },
}

impl Variant {
    pub fn new_list(elements: Vec<Expression>) -> Self {
        Self::List { elements }
    }

    pub fn new_repeated(expression: Expression, size_expression: Expression) -> Self {
        Self::Repeated {
            expression,
            size_expression,
        }
    }
}
