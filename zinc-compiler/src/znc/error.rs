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
    /// The Zinc source code error.
    Source(SourceError),
    /// The output directories creating error.
    DirectoryCreating(OsString, io::Error),
    /// The witness template JSON file writing error.
    WitnessTemplateOutput(OsString, OutputError),
    /// The public data template JSON file writing error.
    PublicDataTemplateOutput(OsString, OutputError),
    /// The storage JSON file writing error.
    StorageOutput(OsString, OutputError),
    /// The bytecode binary file writing error.
    BytecodeOutput(OsString, OutputError),
}

impl From<SourceError> for Error {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(inner) => write!(f, "{}", inner),
            Self::DirectoryCreating(path, inner) => {
                write!(f, "directory `{:?}` creating: {}", path, inner)
            }
            Self::WitnessTemplateOutput(path, inner) => {
                write!(f, "witness template file `{:?}` output: {}", path, inner)
            }
            Self::PublicDataTemplateOutput(path, inner) => write!(
                f,
                "public data template file `{:?}` output: {}",
                path, inner
            ),
            Self::StorageOutput(path, inner) => {
                write!(f, "storage file `{:?}` output: {}", path, inner)
            }
            Self::BytecodeOutput(path, inner) => {
                write!(f, "bytecode file `{:?}` output: {}", path, inner)
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
