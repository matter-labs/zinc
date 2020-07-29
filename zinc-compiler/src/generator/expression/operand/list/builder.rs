//!
//! The generator expression list operand builder.
//!

use crate::generator::expression::operand::list::Expression as ListExpression;
use crate::generator::expression::Expression as GeneratorExpression;

///
/// The generator expression list operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The function argument expression array.
    expressions: Vec<GeneratorExpression>,
}

impl Builder {
    ///
    /// Pushes a function argument expression.
    ///
    pub fn push_expression(&mut self, value: GeneratorExpression) {
        self.expressions.push(value);
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(self) -> ListExpression {
        ListExpression::new(self.expressions)
    }
}
