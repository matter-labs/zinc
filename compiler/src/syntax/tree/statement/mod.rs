//!
//! The statement.
//!

mod debug;
mod r#let;
mod r#loop;
mod require;

pub use self::debug::Builder as DebugBuilder;
pub use self::debug::Debug;
pub use self::r#let::Builder as LetBuilder;
pub use self::r#let::Let;
pub use self::r#loop::Builder as LoopBuilder;
pub use self::r#loop::Loop;
pub use self::require::Builder as RequireBuilder;
pub use self::require::Require;

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::Expression;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "statement")]
pub enum Statement {
    Require(Require),
    Let(Let),
    Debug(Debug),
    Loop(Loop),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Require(statement) => write!(f, "{}", statement),
            Self::Let(statement) => write!(f, "{}", statement),
            Self::Debug(statement) => write!(f, "{}", statement),
            Self::Loop(statement) => write!(f, "{}", statement),
            Self::Expression(statement) => write!(f, "{}", statement),
        }
    }
}
