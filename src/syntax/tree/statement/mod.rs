//!
//! The statement.
//!

mod debug;
mod r#let;
mod require;

pub use self::debug::Builder as DebugBuilder;
pub use self::debug::Debug;
pub use self::r#let::Builder as LetBuilder;
pub use self::r#let::Let;
pub use self::require::Builder as RequireBuilder;
pub use self::require::Require;

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "statement")]
pub enum Statement {
    Require(Require),
    Let(Let),
    Debug(Debug),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Require(require) => write!(f, "{}", require),
            Statement::Let(r#let) => write!(f, "{}", r#let),
            Statement::Debug(debug) => write!(f, "{}", debug),
        }
    }
}
