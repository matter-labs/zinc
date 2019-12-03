//!
//! The let statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub location: Location,
    pub identifier: Identifier,
    pub is_mutable: bool,
    pub r#type: Option<Type>,
    pub expression: Expression,
}

impl Let {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Option<Type>,
        expression: Expression,
    ) -> Self {
        Self {
            location,
            identifier,
            is_mutable,
            r#type,
            expression,
        }
    }
}
