//!
//! The array expression variant.
//!

use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The array expression variant.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// The array list variant.
    List {
        /// The array element expressions.
        elements: Vec<ExpressionTree>,
    },
    /// The array with a repeated value variant.
    Repeated {
        /// The expression which is repeated.
        expression: ExpressionTree,
        /// The size expression specifying how many times the expression above is repeated.
        size_expression: ExpressionTree,
    },
}

impl Variant {
    ///
    /// Creates an array expression with separate values.
    ///
    pub fn new_list(elements: Vec<ExpressionTree>) -> Self {
        Self::List { elements }
    }

    ///
    /// Creates an array expression with a single repeated value.
    ///
    pub fn new_repeated(expression: ExpressionTree, size_expression: ExpressionTree) -> Self {
        Self::Repeated {
            expression,
            size_expression,
        }
    }
}
