//!
//! The array expression variant.
//!

use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    List {
        elements: Vec<ExpressionTree>,
    },
    Repeated {
        expression: ExpressionTree,
        size_expression: ExpressionTree,
    },
}

impl Variant {
    pub fn new_list(elements: Vec<ExpressionTree>) -> Self {
        Self::List { elements }
    }

    pub fn new_repeated(expression: ExpressionTree, size_expression: ExpressionTree) -> Self {
        Self::Repeated {
            expression,
            size_expression,
        }
    }
}
