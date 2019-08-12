//!
//! The syntax type keyword.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum Type {
    //    Void,
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
