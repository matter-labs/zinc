//!
//! The input.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Input {
    #[serde(skip_serializing)]
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Input {
    pub fn new(location: Location, identifier: Identifier, r#type: Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}
