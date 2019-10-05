//!
//! The expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<ExpressionElement>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_location_if_unset(&mut self, value: Location) {
        if self.location.is_none() {
            self.location = Some(value);
        }
    }

    pub fn push_operand(&mut self, location: Location, operand: ExpressionOperand) {
        self.elements.push(ExpressionElement::new(
            location,
            ExpressionObject::Operand(operand),
        ));
    }

    pub fn push_operator(&mut self, location: Location, operator: ExpressionOperator) {
        self.elements.push(ExpressionElement::new(
            location,
            ExpressionObject::Operator(operator),
        ));
    }

    pub fn extend_with_expression(&mut self, expression: Expression) {
        self.elements.extend(expression.elements)
    }

    pub fn extend_with_expressions(&mut self, expressions: Vec<Expression>) {
        for expression in expressions.into_iter() {
            self.elements.extend(expression.elements)
        }
    }

    pub fn finish(mut self) -> Expression {
        Expression::new(
            self.location.take().expect("Missing location"),
            self.elements,
        )
    }
}
