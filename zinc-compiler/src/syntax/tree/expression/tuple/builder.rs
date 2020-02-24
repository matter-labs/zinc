//!
//! The tuple expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::element::Element as ExpressionElement;
use crate::syntax::tree::expression::object::Object as ExpressionObject;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tuple::Expression as TupleExpression;
use crate::syntax::tree::expression::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<Expression>,
    has_comma: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: Expression) {
        self.elements.push(expression);
    }

    pub fn set_comma(&mut self) {
        self.has_comma = true;
    }

    pub fn finish(mut self) -> Expression {
        match (self.elements.len(), self.has_comma) {
            (0, false) => {
                let mut builder = ExpressionBuilder::default();
                let location = self.location.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                        "location"
                    )
                });
                builder.set_location(location);
                builder.push_operand(location, ExpressionOperand::Unit);
                builder.finish()
            }
            (1, false) => {
                let mut builder = ExpressionBuilder::default();
                let location = self.location.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                        "location"
                    )
                });
                builder.set_location(location);
                builder.extend_with_expressions(self.elements);
                builder.finish()
            }
            (_size, _has_comma) => {
                let location = self.location.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                        "location"
                    )
                });
                Expression::new(
                    location,
                    vec![ExpressionElement::new(
                        location,
                        ExpressionObject::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                            location,
                            self.elements,
                        ))),
                    )],
                )
            }
        }
    }
}
