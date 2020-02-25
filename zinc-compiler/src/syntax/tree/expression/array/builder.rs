//!
//! The array expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::array::Expression as ArrayExpression;
use crate::syntax::tree::expression::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<Expression>,
    size_expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: Expression) {
        self.elements.push(expression);
    }

    pub fn set_size_expression(&mut self, value: Expression) {
        self.size_expression = Some(value);
    }

    pub fn finish(mut self) -> ArrayExpression {
        ArrayExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.elements,
            self.size_expression.take(),
        )
    }
}
