//!
//! The block expression builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Statement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    statements: Vec<Statement>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_statement(&mut self, value: Statement) {
        self.statements.push(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> BlockExpression {
        BlockExpression::new(
            self.location.take().expect("Missing location"),
            self.statements,
            self.expression.take(),
        )
    }
}
