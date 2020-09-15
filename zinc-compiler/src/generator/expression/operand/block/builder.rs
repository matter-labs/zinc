//!
//! The generator expression block operand builder.
//!

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::statement::Statement;

///
/// The generator expression block operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The block statements.
    statements: Vec<Statement>,
    /// The optional block expressions, whose type is defaulted to `()` if unset.
    expression: Option<GeneratorExpression>,
}

impl Builder {
    ///
    /// Pushes a block statement.
    ///
    pub fn push_statement(&mut self, value: Statement) {
        self.statements.push(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_expression(&mut self, value: GeneratorExpression) {
        self.expression = Some(value);
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(self) -> BlockExpression {
        BlockExpression::new(self.statements, self.expression)
    }
}
