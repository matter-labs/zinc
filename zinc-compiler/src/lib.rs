//!
//! The Zinc compiler library.
//!

#![allow(clippy::large_enum_variant)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::too_many_arguments)]

pub(crate) mod error;
pub(crate) mod file;
pub(crate) mod generator;
pub(crate) mod lexical;
pub(crate) mod semantic;
pub(crate) mod syntax;

pub use self::error::Error;
pub use self::file::File;
pub use self::generator::bytecode::Bytecode;
pub use self::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
pub use self::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
pub use self::semantic::scope::Scope;
pub use self::syntax::parser::Parser;
pub use self::syntax::tree::Tree;

pub const BASE_BINARY: usize = 2;
pub const BASE_OCTAL: usize = 8;
pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;

pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_INDEX: usize = 64;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;
pub const BITLENGTH_SHA256_HASH: usize = 256;
pub const BITLENGTH_BLAKE2S_HASH: usize = 256;

pub const LIMIT_PEDERSEN_HASH_INPUT_BITS: usize = 512;
pub const LIMIT_SCHNORR_MESSAGE_BYTES: usize = 31;
pub const LIMIT_SCHNORR_MESSAGE_BITS: usize = LIMIT_SCHNORR_MESSAGE_BYTES * BITLENGTH_BYTE;

pub static PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";
pub static PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";
pub static PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";
pub static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
pub static PANIC_MUTEX_SYNC: &str = "Mutexes never panic";
pub static PANIC_FILE_INDEX: &str = "File record always exists";
pub static PANIC_BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
