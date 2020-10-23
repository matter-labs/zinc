//!
//! The source code file error.
//!

use std::ffi::OsString;
use std::fmt;
use std::io;

///
/// The source code file error.
///
#[derive(Debug)]
pub enum Error {
    /// The file opening error.
    Opening(io::Error),
    /// The file metadata getting error.
    Metadata(io::Error),
    /// The file reading error.
    Reading(io::Error),
    /// The file has no extension.
    ExtensionNotFound,
    /// The file extension is not the one we are looking for.
    ExtensionInvalid(OsString),
    /// The file has no stem, that is, name without the extension.
    StemNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opening(inner) => write!(f, "opening: {}", inner),
            Self::Metadata(inner) => write!(f, "metadata: {}", inner),
            Self::Reading(inner) => write!(f, "reading: {}", inner),
            Self::ExtensionNotFound => write!(f, "file extension not found"),
            Self::ExtensionInvalid(extension) => {
                write!(f, "file extension `{:?}` is invalid", extension)
            }
            Self::StemNotFound => write!(f, "file name not found"),
        }
    }
}
