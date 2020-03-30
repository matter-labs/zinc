//!
//! The tuple expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::expression::tuple::Expression as TupleExpression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<ExpressionTree>,
    has_comma: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: ExpressionTree) {
        self.elements.push(expression);
    }

    pub fn set_comma(&mut self) {
        self.has_comma = true;
    }

    pub fn finish(mut self) -> ExpressionOperand {
        match (self.elements.len(), self.has_comma) {
            (0, false) => ExpressionOperand::Unit,
            (1, false) => {
                ExpressionOperand::Parenthesized(self.elements.pop().map(Box::new).unwrap())
            }
            (_size, _has_comma) => {
                let location = self.location.take().unwrap_or_else(|| {
                    panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location")
                });
                ExpressionOperand::Tuple(TupleExpression::new(location, self.elements))
            }
        }
    }
}
