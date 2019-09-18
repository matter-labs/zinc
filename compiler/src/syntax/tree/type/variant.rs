//!
//! The type variant.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum Variant {
    Void,
    Bool,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
}

impl Variant {
    pub fn uint(bitlength: usize) -> Self {
        Self::Uint { bitlength }
    }

    pub fn int(bitlength: usize) -> Self {
        Self::Int { bitlength }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),
        }
    }
}
