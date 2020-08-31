//!
//! The project `data` directory.
//!

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

///
/// The project `data` directory.
///
pub struct Directory {}

///
/// The project `data` directory error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The directory creating error.
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    /// The directory removing error.
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
}

impl Directory {
    ///
    /// If the `path` does not end with the directory subpath, appends the subpath to the `path`.
    ///
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::directory::DATA) {
            path.push(PathBuf::from(zinc_const::directory::DATA));
        }
        path
    }

    ///
    /// Creates a directory with all its parent directories if it does not exist.
    ///
    pub fn create(path: &PathBuf) -> Result<(), Error> {
        fs::create_dir_all(&Self::path(path)).map_err(Error::Creating)
    }

    ///
    /// Removes the directory with all its child directories.
    ///
    pub fn remove(path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::directory::DATA) {
            path.push(PathBuf::from(zinc_const::directory::DATA));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(Error::Removing)?;
        }

        Ok(())
    }
}
