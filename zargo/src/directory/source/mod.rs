//!
//! The project `src` directory.
//!

pub mod circuit;
pub mod contract;

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

///
/// The project `src` directory.
///
pub struct Directory {}

///
/// The project `src` directory error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The directory creating error.
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
}

impl Directory {
    ///
    /// If the `path` does not end with the directory subpath, appends the subpath to the `path`.
    ///
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::directory::SOURCE) {
            path.push(PathBuf::from(zinc_const::directory::SOURCE));
        }
        path
    }

    ///
    /// Creates a directory with all its parent directories if it does not exist.
    ///
    pub fn create(path: &PathBuf) -> Result<(), Error> {
        fs::create_dir_all(&Self::path(path)).map_err(Error::Creating)
    }
}
