//!
//! The parser library.
//!

mod error;
mod lexical;
mod semantic;
mod syntax;

pub use self::error::Error;
pub use self::semantic::BinaryAnalyzer;
pub use self::syntax::CircuitProgram;
pub use self::syntax::Parser;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;
pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_INDEX: usize = 64;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;
