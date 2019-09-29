//!
//! The input.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

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

    pub fn bitlength(&self) -> usize {
        match self.r#type.variant {
            TypeVariant::Boolean => 1,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Field => crate::SIZE_FIELD,
            _ => 0,
        }
    }
}
