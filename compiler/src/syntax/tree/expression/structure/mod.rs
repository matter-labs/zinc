//!
//! The structure expression.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub path: syntax::Expression,
    pub fields: Vec<(Identifier, syntax::Expression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        path: syntax::Expression,
        fields: Vec<(Identifier, syntax::Expression)>,
    ) -> Self {
        Self {
            location,
            path,
            fields,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "struct {{ {} }}",
            self.fields
                .iter()
                .map(|(identifier, expression)| format!("{}: {}", identifier, expression))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
