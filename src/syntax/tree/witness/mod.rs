//!
//! The witness.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Witness {
    identifier: Identifier,
    r#type: Type,
}

impl Witness {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self { identifier, r#type }
    }
}
