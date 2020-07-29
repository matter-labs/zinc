//!
//! The source code directory error.
//!

use std::fmt;
use std::io;

///
/// The source code directory error.
///
#[derive(Debug)]
pub enum Error {
    /// The directory opening error.
    Reading(io::Error),
    /// The directory entry getting error.
    DirectoryEntry(io::Error),
    /// The directory name getting error.
    StemNotFound,
    /// The application entry not found. Only for the root directory.
    ApplicationEntryNotFound,
    /// The module entry not found.
    ModuleEntryNotFound,
    /// The application entry file is deeper than the root directory.
    ApplicationEntryBeyondRoot,
    /// The module entry is in the root directory. Only the application entry allowed there.
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
                zinc_const::file_names::APPLICATION_ENTRY,
                zinc_const::extensions::SOURCE,
            ),
            Self::ModuleEntryNotFound => write!(
                f,
                "the module entry file `{}.{}` is missing",
                zinc_const::file_names::MODULE_ENTRY,
                zinc_const::extensions::SOURCE,
            ),
            Self::ApplicationEntryBeyondRoot => write!(
                f,
                "the application entry file `{}.{}` is beyond the source code root",
                zinc_const::file_names::APPLICATION_ENTRY,
                zinc_const::extensions::SOURCE,
            ),
            Self::ModuleEntryInRoot => write!(
                f,
                "the module entry file `{}.{}` cannot be the application entry",
                zinc_const::file_names::MODULE_ENTRY,
                zinc_const::extensions::SOURCE,
            ),
        }
    }
}
