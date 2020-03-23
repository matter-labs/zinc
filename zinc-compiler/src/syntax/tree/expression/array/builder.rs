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
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location"));

        match self.size_expression.take() {
            Some(size_expression) => ArrayExpression::new_repeated(
                location,
                self.elements.pop().unwrap_or_else(|| {
                    panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "expression")
                }),
                size_expression,
            ),
            None => ArrayExpression::new_list(location, self.elements),
        }
    }
}
