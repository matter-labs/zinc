//!
//! The source code module error.
//!

use std::fmt;
use std::io;

use crate::source::directory::error::Error as DirectoryError;
use crate::source::file::error::Error as FileError;

///
/// The source code module error.
///
#[derive(Debug)]
pub enum Error {
    /// File metadata getting error.
    FileMetadata(io::Error),
    /// Failed to get the file type, that is, file or directory.
    FileTypeUnknown,

    /// The source code file error.
    File(FileError),
    /// The source code directory error.
    Directory(DirectoryError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileMetadata(inner) => write!(f, "file metadata: `{}`", inner),
            Self::FileTypeUnknown => write!(f, "file type is neither file nor directory"),

            Self::File(inner) => write!(f, "file {}", inner),
            Self::Directory(inner) => write!(f, "directory {}", inner),
        }
    }
}
