//!
//! The block expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::statement::local_fn::Statement as FunctionLocalStatement;

///
/// The block expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The function block statements.
    statements: Vec<FunctionLocalStatement>,
    /// The optional last statement, which is the result of the block.
    expression: Option<ExpressionTree>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location_if_unset(&mut self, value: Location) {
        if self.location.is_none() {
            self.location = Some(value);
        }
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_statement(&mut self, value: FunctionLocalStatement) {
        self.statements.push(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_expression(&mut self, value: ExpressionTree) {
        self.expression = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> BlockExpression {
        BlockExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.statements,
            self.expression.take(),
        )
    }
}
