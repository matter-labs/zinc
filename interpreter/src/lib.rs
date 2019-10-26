//!
//! The interpreter library.
//!

#![allow(clippy::large_enum_variant)]

mod element;
mod error;
mod interpreter;
mod scope;
mod tests;

pub use self::element::Value;
pub use self::error::Error;
pub use self::interpreter::Interpreter;
