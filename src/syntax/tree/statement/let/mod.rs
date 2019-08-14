//!
//! The let statement.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::lexical::Token;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Let {
    identifier: Identifier,
    r#type: Option<Type>,
    expression: Vec<Token>,
    is_mutable: bool,
}

impl Let {
    pub fn new(
        identifier: Identifier,
        r#type: Option<Type>,
        expression: Vec<Token>,
        is_mutable: bool,
    ) -> Self {
        Self {
            identifier,
            r#type,
            expression,
            is_mutable,
        }
    }
}
