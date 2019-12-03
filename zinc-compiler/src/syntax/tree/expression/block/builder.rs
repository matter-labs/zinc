//!
//! The block expression builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::InnerStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    statements: Vec<InnerStatement>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_statement(&mut self, value: InnerStatement) {
        self.statements.push(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> BlockExpression {
        BlockExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.statements,
            self.expression.take(),
        )
    }
}
