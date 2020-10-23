//!
//! The Zinc source code string representation.
//!

pub mod directory;
pub mod error;
pub mod file;

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

pub use self::directory::Directory;
pub use self::error::Error;
pub use self::file::File;

///
/// The Zinc source code JSON representation.
///
/// This representation is used for the Zandbox server requests, where the project source code must
/// be bundled into JSON to be passed via a single request.
///
/// The compiler understands this format as well. Source code may be passed to the compiler either
/// in this representation, or just by the path to the source code folder, which is usually `/src`.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    /// The virtual file string data.
    File(File),
    /// The virtual directory string data.
    Directory(Directory),
}

impl Source {
    ///
    /// Initializes a virtual application module representation from the file system.
    ///
    pub fn try_from_path(path: &PathBuf, is_entry: bool) -> Result<Self, Error> {
        let file_type = fs::metadata(path).map_err(Error::FileMetadata)?.file_type();

        if file_type.is_dir() {
            return Directory::try_from_path(path, is_entry).map(Self::Directory);
        }

        if file_type.is_file() {
            return File::try_from_path(path).map(Self::File);
        }

        Err(Error::FileTypeUnknown)
    }

    ///
    /// Gets the file or directory name.
    ///
    pub fn name(&self) -> &str {
        match self {
            Self::File(inner) => inner.name.as_str(),
            Self::Directory(inner) => inner.name.as_str(),
        }
    }
}
