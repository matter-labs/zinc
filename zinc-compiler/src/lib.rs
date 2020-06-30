//!
//! The Zinc compiler library.
//!

//#![deny(missing_docs)]
//#![deny(clippy::missing_docs_in_private_items)]

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
pub use self::source::directory::Directory as SourceDirectory;
pub use self::source::directory::DirectoryString as SourceDirectoryString;
pub use self::source::error::Error as SourceError;
pub use self::source::file::File as SourceFile;
pub use self::source::file::FileString as SourceFileString;
pub use self::source::Source;
pub use self::source::SourceString;
