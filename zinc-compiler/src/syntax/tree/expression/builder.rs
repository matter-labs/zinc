//!
//! The expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
use crate::syntax::tree::expression::element::Element as ExpressionElement;
use crate::syntax::tree::expression::object::Object as ExpressionObject;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::Expression;

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

    pub fn push_auxiliary(&mut self, location: Location, auxiliary: ExpressionAuxiliary) {
        self.elements.push(ExpressionElement::new(
            location,
            ExpressionObject::Auxiliary(auxiliary),
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
