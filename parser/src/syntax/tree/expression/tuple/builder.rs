//!
//! The tuple expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::TupleExpression;

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
                let location = self.location.take().expect("Missing location");
                builder.set_location(location);
                builder.push_operand(location, ExpressionOperand::Unit);
                builder.finish()
            }
            (1, false) => {
                let mut builder = ExpressionBuilder::default();
                let location = self.location.take().expect("Missing location");
                builder.set_location(location);
                builder.extend_with_expressions(self.elements);
                builder.finish()
            }
            (_size, true) => {
                let location = self.location.take().expect("Missing location");
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
            _ => panic!("Always checked by the branches above"),
        }
    }
}
