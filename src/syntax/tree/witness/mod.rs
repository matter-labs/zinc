//!
//! The witness.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Witness {
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Witness {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self { identifier, r#type }
    }
}
