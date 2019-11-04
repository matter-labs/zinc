//!
//! The fn statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Field;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Fn {
    pub location: Location,
    pub identifier: Identifier,
    pub arguments: Vec<Field>,
    pub return_type: Type,
    pub body: BlockExpression,
}

impl Fn {
    pub fn new(
        location: Location,
        identifier: Identifier,
        arguments: Vec<Field>,
        return_type: Type,
        body: BlockExpression,
    ) -> Self {
        Self {
            location,
            identifier,
            arguments,
            return_type,
            body,
        }
    }
}

impl fmt::Display for Fn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn {}({}) -> {} {{ {} }}",
            self.identifier,
            self.arguments
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
            self.body,
        )
    }
}
