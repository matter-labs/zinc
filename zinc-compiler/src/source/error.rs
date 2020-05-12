//!
//! The source code error.
//!

use std::ffi::OsString;
use std::fmt;

use crate::source::module::error::Error as ModuleError;

#[derive(Debug)]
pub enum Error {
    EntrySourceFileNotFound,

    RootModule { path: OsString, inner: ModuleError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EntrySourceFileNotFound => write!(
                f,
                "the entry source file `{}.{}` is missing",
                crate::APPLICATION_ENTRY_FILE_NAME,
                crate::SOURCE_FILE_EXTENSION
            ),

            Self::RootModule { path, inner } => write!(f, "root module `{:?}` {}", path, inner),
        }
    }
}
