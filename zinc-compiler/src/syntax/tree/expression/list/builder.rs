//!
//! The list expression builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::list::Expression as ListExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    expressions: Vec<ExpressionTree>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, value: ExpressionTree) {
        self.expressions.push(value);
    }

    pub fn finish(mut self) -> ListExpression {
        ListExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.expressions,
        )
    }
}
