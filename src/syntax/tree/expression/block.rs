//!
//! The block expression.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Statement;

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub statements: Vec<Statement>,
    pub expression: Option<Box<syntax::Expression>>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{};", statement)?;
        }
        if let Some(ref expression) = self.expression {
            write!(f, "{}", expression)?;
        }
        Ok(())
    }
}
