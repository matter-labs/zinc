//!
//! The Zinc compiler library.
//!

pub(crate) mod error;
pub(crate) mod generator;
pub(crate) mod lexical;
pub(crate) mod semantic;
pub(crate) mod source;
pub(crate) mod syntax;

pub use self::error::Error;
pub use self::generator::module::Module;
pub use self::generator::state::State;
pub use self::generator::IBytecodeWritable;
pub use self::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
pub use self::semantic::scope::Scope;
pub use self::source::directory::Directory as SourceDirectory;
pub use self::source::error::Error as SourceError;
pub use self::source::file::File as SourceFile;
pub use self::source::Source;
