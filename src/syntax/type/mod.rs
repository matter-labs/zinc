//!
//! The syntax type keyword.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    //    Void,
    Uint(usize),
    Int(usize),
    Field,
    Bool,
    //    Struct(Identifier, Vec<(Identifier, Type)>),
    //    Enum(Identifier, Vec<Identifier>),
    //    Tuple(Vec<Type>),
    //    MemoryVector(Box<Type>, usize),
    //    StorageVector(Box<Type>, usize),
}
