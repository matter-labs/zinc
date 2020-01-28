//!
//! The Zinc compiler library.
//!

mod error;
mod lexical;
mod semantic;
mod syntax;

pub use self::error::Error;
pub use self::semantic::BinaryAnalyzer;
pub use self::semantic::Bytecode;
pub use self::semantic::LibraryAnalyzer;
pub use self::semantic::Scope;
pub use self::syntax::Parser;
pub use self::syntax::SyntaxTree;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;
pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;

pub const SHA256_HASH_SIZE_BITS: usize = 256;
