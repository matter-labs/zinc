//!
//! The block expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::FunctionLocalStatement;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub statements: Vec<FunctionLocalStatement>,
    pub expression: Option<Box<syntax::Expression>>,
}

impl Expression {
    pub fn new(
        location: Location,
        statements: Vec<FunctionLocalStatement>,
        expression: Option<syntax::Expression>,
    ) -> Self {
        Self {
            location,
            statements,
            expression: expression.map(Box::new),
        }
    }
}
