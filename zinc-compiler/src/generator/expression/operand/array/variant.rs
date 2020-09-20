//!
//! The generator expression array operand variant.
//!

use crate::generator::expression::Expression as GeneratorExpression;

///
/// The generator expression array operand variant.
///
#[derive(Debug, Clone)]
pub enum Variant {
    /// The list variant, where each item is translated separately.
    List {
        /// The array element expressions.
        expressions: Vec<GeneratorExpression>,
    },
    /// The repeated variant, where a single element is repeated many times.
    Repeated {
        /// The array element to repeat.
        expression: GeneratorExpression,
        /// The number of times to repeat the `expression`.
        size: usize,
    },
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_list(expressions: Vec<GeneratorExpression>) -> Self {
        Self::List { expressions }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_repeated(expression: GeneratorExpression, size: usize) -> Self {
        Self::Repeated { expression, size }
    }
}
