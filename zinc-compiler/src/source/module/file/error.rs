//!
//! The source code file error.
//!

use std::ffi::OsString;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Opening(io::Error),
    Metadata(io::Error),
    Reading(io::Error),
    ExtensionNotFound,
    ExtensionInvalid(OsString),
    StemNotFound,

    Compiling(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Opening(inner) => write!(f, "opening: {}", inner),
            Self::Metadata(inner) => write!(f, "metadata: {}", inner),
            Self::Reading(inner) => write!(f, "reading: {}", inner),
            Self::ExtensionNotFound => write!(f, "file extension not found"),
            Self::ExtensionInvalid(extension) => {
                write!(f, "file extension `{:?}` is invalid", extension)
            }
            Self::StemNotFound => write!(f, "file name not found"),
            Self::Compiling(inner) => write!(f, "compiling: {}", inner),
        }
    }
}
