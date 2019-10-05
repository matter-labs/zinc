//!
//! The witness.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Witness {
    #[serde(skip_serializing)]
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
}

impl Witness {
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
            TypeVariant::IntegerUnsigned { bitlength } => bitlength,
            TypeVariant::IntegerSigned { bitlength } => bitlength,
            TypeVariant::Field => crate::SIZE_FIELD,
            _ => panic!("Always checked by the branches above"),
        }
    }
}
