//!
//! The block expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::InnerStatement;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub statements: Vec<InnerStatement>,
    pub expression: Option<Box<syntax::Expression>>,
}

impl Expression {
    pub fn new(
        location: Location,
        statements: Vec<InnerStatement>,
        expression: Option<syntax::Expression>,
    ) -> Self {
        Self {
            location,
            statements,
            expression: expression.map(Box::new),
        }
    }
}
