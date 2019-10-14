//!
//! The interpreter library.
//!

mod element;
mod error;
mod interpreter;
mod scope;
mod tests;

pub use self::error::Error;
pub use self::interpreter::Interpreter;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;

pub const MAX_VALUE_BYTE: usize = 256;

pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = r1cs::BITLENGTH_FIELD;
pub const BITLENGTH_FIELD_PADDED: usize = 256;
