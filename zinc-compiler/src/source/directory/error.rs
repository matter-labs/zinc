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
    ApplicationEntryNotFound,
    ModuleEntryNotFound,
    ApplicationEntryBeyondRoot,
    ModuleEntryInRoot,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reading(inner) => write!(f, "reading: `{}`", inner),
            Self::DirectoryEntry(inner) => write!(f, "directory entry: `{}`", inner),
            Self::StemNotFound => write!(f, "directory name not found"),
            Self::ApplicationEntryNotFound => write!(
                f,
                "the application entry file `{}.{}` is missing",
                crate::APPLICATION_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION,
            ),
            Self::ModuleEntryNotFound => write!(
                f,
                "the module entry file `{}.{}` is missing",
                crate::MODULE_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION,
            ),
            Self::ApplicationEntryBeyondRoot => write!(
                f,
                "the application entry file `{}.{}` is beyond the source code root",
                crate::APPLICATION_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION,
            ),
            Self::ModuleEntryInRoot => write!(
                f,
                "the module entry file `{}.{}` cannot be the application entry",
                crate::MODULE_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION,
            ),
        }
    }
}
