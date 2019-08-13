//!
//! The syntax input.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Input {
    identifier: Identifier,
    r#type: Type,
}

impl Input {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self { identifier, r#type }
    }
}
