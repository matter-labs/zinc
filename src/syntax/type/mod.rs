//!
//! The syntax type keyword.
//!

mod keyword;

pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;

use crate::syntax::Identificator;

#[derive(Debug)]
pub enum Type {
    Void,
    Uint(usize),
    Int(usize),
    Field,
    Bool,
    Struct(Vec<()>),
    EnumVec(Vec<Identificator>),
    Tuple(Vec<Type>),
    MemoryVector(Box<Type>, usize),
    StorageVector(Box<Type>, usize),
}
