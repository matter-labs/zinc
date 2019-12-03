//!
//! The array expression builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::ArrayExpression;
use crate::syntax::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: Expression) {
        self.elements.push(expression);
    }

    pub fn fill(&mut self, size: IntegerLiteral) {
        let expression = self.elements.pop().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "expression"
            )
        });
        let size: usize = size.into();
        self.elements = vec![expression; size];
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
        )
    }
}
