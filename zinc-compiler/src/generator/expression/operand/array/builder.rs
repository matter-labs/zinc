//!
//! The generator expression array operand builder.
//!

use crate::generator::expression::operand::array::Expression as ArrayExpression;
use crate::generator::expression::Expression as GeneratorExpression;

///
/// The generator expression array operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The array expressions.
    expressions: Vec<GeneratorExpression>,
    /// The explicit array size. If set, the array is created as repeated.
    size: Option<usize>,
}

impl Builder {
    ///
    /// Pushes an array element expression.
    ///
    pub fn push_expression(&mut self, value: GeneratorExpression) {
        self.expressions.push(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_size(&mut self, value: usize) {
        self.size = Some(value);
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(mut self) -> ArrayExpression {
        match self.size.take() {
            Some(size) => {
                let expression = self.expressions.pop().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        zinc_const::panic::BUILDER_REQUIRES_VALUE,
                        "expression"
                    )
                });

                ArrayExpression::new_repeated(expression, size)
            }
            None => ArrayExpression::new_list(self.expressions),
        }
    }
}
