//!
//! The statement.
//!

mod debug;
mod r#enum;
mod r#let;
mod r#loop;
mod require;
mod r#struct;
mod r#type;

pub use self::debug::Builder as DebugBuilder;
pub use self::debug::Debug;
pub use self::r#enum::Builder as EnumBuilder;
pub use self::r#enum::Enum;
pub use self::r#let::Builder as LetBuilder;
pub use self::r#let::Let;
pub use self::r#loop::Builder as LoopBuilder;
pub use self::r#loop::Loop;
pub use self::r#struct::Builder as StructBuilder;
pub use self::r#struct::Struct;
pub use self::r#type::Builder as TypeBuilder;
pub use self::r#type::Type;
pub use self::require::Builder as RequireBuilder;
pub use self::require::Require;

use std::fmt;

use crate::syntax::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Empty,
    Require(Require),
    Let(Let),
    Loop(Loop),
    Type(Type),
    Struct(Struct),
    Enum(Enum),
    Debug(Debug),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ";"),
            Self::Require(statement) => write!(f, "{}", statement),
            Self::Let(statement) => write!(f, "{}", statement),
            Self::Loop(statement) => write!(f, "{}", statement),
            Self::Type(statement) => write!(f, "{}", statement),
            Self::Struct(statement) => write!(f, "{}", statement),
            Self::Enum(statement) => write!(f, "{}", statement),
            Self::Debug(statement) => write!(f, "{}", statement),
            Self::Expression(statement) => write!(f, "{}", statement),
        }
    }
}
