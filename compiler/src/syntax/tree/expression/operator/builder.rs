//!
//! The operator expression builder.
//!

use crate::lexical::Location;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionElement;
use crate::syntax::OperatorExpressionObject;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<OperatorExpressionElement>,
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

    pub fn push_operand(&mut self, location: Location, operand: OperatorExpressionOperand) {
        self.elements.push(OperatorExpressionElement::new(
            location,
            OperatorExpressionObject::Operand(operand),
        ));
    }

    pub fn push_operator(&mut self, location: Location, operator: OperatorExpressionOperator) {
        self.elements.push(OperatorExpressionElement::new(
            location,
            OperatorExpressionObject::Operator(operator),
        ));
    }

    pub fn append_expression(&mut self, mut expression: OperatorExpression) {
        self.elements.append(expression.elements.as_mut())
    }

    pub fn finish(mut self) -> OperatorExpression {
        OperatorExpression::new(
            self.location.take().expect("Missing location"),
            self.elements,
        )
    }
}
