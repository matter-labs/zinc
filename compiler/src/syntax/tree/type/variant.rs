//!
//! The type variant.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::TypeVariant;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
pub enum Variant {
    Void,
    Boolean,
    Uint {
        bitlength: usize,
    },
    Int {
        bitlength: usize,
    },
    Field,
    Array {
        type_variant: Box<Self>,
        size: usize,
    },
}

impl Variant {
    pub fn uint(bitlength: usize) -> Self {
        Self::Uint { bitlength }
    }

    pub fn int(bitlength: usize) -> Self {
        Self::Int { bitlength }
    }

    pub fn array(type_variant: TypeVariant, size: usize) -> Self {
        Self::Array {
            type_variant: Box::new(type_variant),
            size,
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),
            Self::Array { type_variant, size } => write!(f, "[{}; {}]", type_variant, size),
        }
    }
}
