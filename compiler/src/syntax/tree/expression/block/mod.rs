//!
//! The block expression.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Statement;

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Expression {
    #[serde(skip_serializing)]
    pub location: Location,
    pub statements: Vec<Statement>,
    pub expression: Option<Box<syntax::Expression>>,
}

impl Expression {
    pub fn new(
        location: Location,
        statements: Vec<Statement>,
        expression: Option<syntax::Expression>,
    ) -> Self {
        Self {
            location,
            statements,
            expression: expression.map(Box::new),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ ")?;
        for statement in self.statements.iter() {
            write!(f, "{};", statement)?;
        }
        if let Some(ref expression) = self.expression {
            write!(f, "{}", expression)?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}
