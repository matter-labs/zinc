//!
//! The block expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    statements: Vec<FunctionLocalStatement>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_statement(&mut self, value: FunctionLocalStatement) {
        self.statements.push(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> BlockExpression {
        BlockExpression::new(
            self.location
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location")),
            self.statements,
            self.expression.take(),
        )
    }
}
