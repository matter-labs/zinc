//!
//! The Zinc compiler binary error.
//!

use std::ffi::OsString;
use std::fmt;
use std::io;

use zinc_compiler::SourceError;

///
/// The Zinc compiler binary error.
///
pub enum Error {
    /// The manifest file error.
    Manifest(zinc_manifest::Error),
    /// The Zinc source code error.
    Source(SourceError),
    /// The output directories creating error.
    DirectoryCreating(OsString, io::Error),
    /// The bytecode binary file writing error.
    BytecodeWriting(OsString, OutputError),
    /// The witness template JSON file writing error.
    InputTemplateWriting(OsString, OutputError),
}

impl From<SourceError> for Error {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest(inner) => write!(f, "{}", inner),
            Self::Source(inner) => write!(f, "{}", inner),
            Self::DirectoryCreating(path, inner) => {
                write!(f, "directory `{:?}` creating: {}", path, inner)
            }
            Self::BytecodeWriting(path, inner) => {
                write!(f, "bytecode file `{:?}` writing: {}", path, inner)
            }
            Self::InputTemplateWriting(path, inner) => {
                write!(f, "input template file `{:?}` writing: {}", path, inner)
            }
        }
    }
}

///
/// The file output error.
///
pub enum OutputError {
    /// The file creating error.
    Creating(std::io::Error),
    /// The file writing error.
    Writing(std::io::Error),
    /// The file removing error.
    Removing(std::io::Error),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creating(inner) => write!(f, "creating: {}", inner),
            Self::Writing(inner) => write!(f, "writing: {}", inner),
            Self::Removing(inner) => write!(f, "removing: {}", inner),
        }
    }
}
