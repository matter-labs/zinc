//!
//! The type variant.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::Type;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
pub enum Variant {
    Void,
    Boolean,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
    Array { r#type: Box<Type>, size: usize },
}

impl Variant {
    pub fn uint(bitlength: usize) -> Self {
        Self::Uint { bitlength }
    }

    pub fn int(bitlength: usize) -> Self {
        Self::Int { bitlength }
    }

    pub fn array(r#type: Type, size: usize) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
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
            Self::Array { r#type, size } => write!(f, "[{}; {}]", r#type, size),
        }
    }
}
