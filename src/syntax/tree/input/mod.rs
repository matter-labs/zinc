//!
//! The input.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Input {
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Input {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self { identifier, r#type }
    }
}
