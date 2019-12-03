//!
//! The fn statement.
//!

mod builder;

pub use self::builder::Builder;

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
