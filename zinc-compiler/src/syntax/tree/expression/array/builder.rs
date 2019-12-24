//!
//! The array expression builder.
//!

use crate::lexical::Location;
use crate::syntax::ArrayExpression;
use crate::syntax::Expression;
use crate::syntax::IntegerLiteral;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<Expression>,
    repeats_count: Option<IntegerLiteral>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: Expression) {
        self.elements.push(expression);
    }

    pub fn set_repeats_count(&mut self, repeats_count: IntegerLiteral) {
        self.repeats_count = Some(repeats_count);
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
            self.repeats_count.take(),
        )
    }
}
