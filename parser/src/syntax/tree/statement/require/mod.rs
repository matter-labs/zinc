//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::lexical::StringLiteral;
use crate::syntax::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Require {
    pub location: Location,
    pub expression: Expression,
    pub annotation: String,
}

impl Require {
    pub fn new(
        location: Location,
        expression: Expression,
        annotation: Option<StringLiteral>,
    ) -> Self {
        let annotation = annotation
            .map(|literal| literal.to_string())
            .unwrap_or_else(|| format!("L{}C{}", location.line, location.column));

        Self {
            location,
            expression,
            annotation,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"require({}, "{}")"#, self.expression, self.annotation)
    }
}
