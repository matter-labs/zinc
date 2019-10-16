//!
//! The interpreter library.
//!

mod element;
mod error;
mod interpreter;
mod scope;
mod tests;

pub use self::element::Value;
pub use self::error::Error;
pub use self::interpreter::Interpreter;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;
pub const MAX_VALUE_BYTE: usize = 256;
