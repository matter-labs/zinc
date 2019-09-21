//!
//! The operator expression builder.
//!

#![allow(dead_code)]

use crate::lexical::Location;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionElement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Option<Vec<OperatorExpressionElement>>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_elements(&mut self, value: Vec<OperatorExpressionElement>) {
        self.elements = Some(value);
    }

    pub fn finish(mut self) -> OperatorExpression {
        OperatorExpression::new(
            self.location.take().expect("Missing location"),
            self.elements.take().unwrap_or_default(),
        )
    }
}
