//!
//! The type.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum Type {
    Void,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
    Bool,
    //    Struct(Identifier, Vec<(Identifier, Type)>),
    //    Enum(Identifier, Vec<Identifier>),
    //    Tuple(Vec<Type>),
    //    MemoryVector(Box<Type>, usize),
    //    StorageVector(Box<Type>, usize),
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        true
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Void => write!(f, "()"),
            Type::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Type::Int { bitlength } => write!(f, "int{}", bitlength),
            Type::Field => write!(f, "field"),
            Type::Bool => write!(f, "bool"),
        }
    }
}
