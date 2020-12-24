//!
//! The Zinc source code string representation.
//!

pub mod directory;
pub mod file;

use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;

pub use self::directory::Directory;
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
    pub fn try_from_path(path: &PathBuf, prefix: &PathBuf, is_entry: bool) -> anyhow::Result<Self> {
        let file_type = fs::metadata(&path)?.file_type();

        if file_type.is_dir() {
            Directory::try_from_path(path, prefix, is_entry).map(Self::Directory)
        } else if file_type.is_file() {
            File::try_from_path(path, prefix).map(Self::File)
        } else {
            Err(Error::FileTypeUnknown).with_context(|| path.to_string_lossy().to_string())
        }
    }

    ///
    /// Writes the module with all inner elements to the disk.
    ///
    pub fn write_to(&self, path: &PathBuf) -> anyhow::Result<()> {
        match self {
            Self::File(inner) => inner.write_to(path),
            Self::Directory(inner) => inner.write_to(path),
        }
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
