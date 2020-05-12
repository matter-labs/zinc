//!
//! The source code module error.
//!

use std::fmt;
use std::io;

use crate::source::module::directory::error::Error as DirectoryError;
use crate::source::module::file::error::Error as FileError;

#[derive(Debug)]
pub enum Error {
    FileMetadata(io::Error),
    FileTypeUnknown,

    File(FileError),
    Directory(DirectoryError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileMetadata(inner) => write!(f, "file metadata: `{}`", inner),
            Self::FileTypeUnknown => write!(f, "file type is neither file nor directory"),

            Self::File(inner) => write!(f, "file {}", inner),
            Self::Directory(inner) => write!(f, "directory {}", inner),
        }
    }
}
