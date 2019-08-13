//!
//! The statement.
//!

mod r#let;
mod require;

pub use self::r#let::Builder as LetBuilder;
pub use self::r#let::Let;
pub use self::require::Builder as RequireBuilder;
pub use self::require::Require;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Statement {
    Require(Require),
    Let(Let),
}
