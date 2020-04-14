//!
//! The source code error.
//!

use std::ffi::OsString;
use std::fmt;

use crate::source::file::error::Error as FileError;

#[derive(Debug)]
pub enum Error {
    EntrySourceFileNotFound,

    File(OsString, FileError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EntrySourceFileNotFound => write!(f, "the 'main.zn' source file is missing"),

            Self::File(path, inner) => write!(f, "file `{:?}` {}", path, inner),
        }
    }
}
