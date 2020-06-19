//!
//! The Zinc compiler library.
//!

#![allow(clippy::large_enum_variant)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub(crate) mod error;
pub(crate) mod generator;
pub(crate) mod lexical;
pub(crate) mod panic;
pub(crate) mod semantic;
pub(crate) mod source;
pub(crate) mod syntax;

pub use self::error::Error;
pub use self::generator::bytecode::entry::Entry;
pub use self::generator::bytecode::Bytecode;
pub use self::generator::program::Program;
pub use self::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
pub use self::semantic::scope::Scope;
pub use self::source::error::Error as SourceError;
pub use self::source::Source;

pub const LIMIT_PEDERSEN_HASH_INPUT_BITS: usize = 512;
pub const LIMIT_SCHNORR_MESSAGE_BYTES: usize = 31;
pub const LIMIT_SCHNORR_MESSAGE_BITS: usize =
    LIMIT_SCHNORR_MESSAGE_BYTES * zinc_const::BITLENGTH_BYTE;

pub static APPLICATION_ENTRY_FILE_NAME: &str = "main";
pub static FUNCTION_MAIN_IDENTIFIER: &str = APPLICATION_ENTRY_FILE_NAME;
pub static MODULE_ENTRY_FILE_NAME: &str = "mod";
pub static SOURCE_FILE_EXTENSION: &str = "zn";
