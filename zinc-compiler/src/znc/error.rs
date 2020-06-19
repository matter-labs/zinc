//!
//! The Zinc compiler binary error.
//!

use std::ffi::OsString;
use std::fmt;
use std::io;

use zinc_compiler::SourceError;

pub enum Error {
    Source(OsString, SourceError),
    DirectoryCreating(OsString, io::Error),
    WitnessTemplateOutput(OsString, OutputError),
    PublicDataTemplateOutput(OsString, OutputError),
    BytecodeOutput(OsString, OutputError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(path, inner) => write!(f, "`{:?}` {}", path, inner),
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
            Self::BytecodeOutput(path, inner) => {
                write!(f, "bytecode file `{:?}` output: {}", path, inner)
            }
        }
    }
}

pub enum OutputError {
    Creating(std::io::Error),
    Writing(std::io::Error),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creating(inner) => write!(f, "creating: {}", inner),
            Self::Writing(inner) => write!(f, "writing: {}", inner),
        }
    }
}
