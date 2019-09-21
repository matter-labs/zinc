//!
//! The block expression builder.
//!

#![allow(dead_code)]

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Statement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    statements: Option<Vec<Statement>>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_statements(&mut self, value: Vec<Statement>) {
        self.statements = Some(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> BlockExpression {
        BlockExpression::new(
            self.location.take().expect("Missing location"),
            self.statements.take().unwrap_or_default(),
            self.expression.take(),
        )
    }
}
