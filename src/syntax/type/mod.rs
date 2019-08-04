//!
//! The syntax type keyword.
//!

mod builder;
mod keyword;

pub use self::builder::Builder;
pub use self::builder::Error as BuilderError;
pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;

use serde_derive::Serialize;

use crate::syntax::Identifier;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Void,
    Uint(usize),
    Int(usize),
    Field,
    Bool,
    Struct(Identifier, Vec<(Identifier, Type)>),
    Enum(Identifier, Vec<Identifier>),
    Tuple(Vec<Type>),
    MemoryVector(Box<Type>, usize),
    StorageVector(Box<Type>, usize),
}
