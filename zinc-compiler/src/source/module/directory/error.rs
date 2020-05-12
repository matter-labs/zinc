//!
//! The source code directory error.
//!

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Reading(io::Error),
    DirectoryEntry(io::Error),
    StemNotFound,
    EntrySourceFileNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Reading(inner) => write!(f, "reading: `{}`", inner),
            Self::DirectoryEntry(inner) => write!(f, "directory entry: `{}`", inner),
            Self::StemNotFound => write!(f, "directory name not found"),
            Self::EntrySourceFileNotFound => write!(
                f,
                "the entry source file `{}.{}` is missing",
                crate::MODULE_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION
            ),
        }
    }
}
