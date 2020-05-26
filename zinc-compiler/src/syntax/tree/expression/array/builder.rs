//!
//! The array expression builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::array::Expression as ArrayExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<ExpressionTree>,
    size_expression: Option<ExpressionTree>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: ExpressionTree) {
        self.elements.push(expression);
    }

    pub fn set_size_expression(&mut self, value: ExpressionTree) {
        self.size_expression = Some(value);
    }

    pub fn finish(mut self) -> ArrayExpression {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        match self.size_expression.take() {
            Some(size_expression) => ArrayExpression::new_repeated(
                location,
                self.elements.pop().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        crate::panic::BUILDER_REQUIRES_VALUE,
                        "size expression"
                    )
                }),
                size_expression,
            ),
            None => ArrayExpression::new_list(location, self.elements),
        }
    }
}
