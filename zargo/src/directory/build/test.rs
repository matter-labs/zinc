//!
//! The project `build/test` directory.
//!

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

///
/// The project `build/test/` directory.
///
pub struct Directory {}

///
/// The project `build/test/` directory error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The directory creating error.
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    /// The directory removing error.
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
    /// The directory reading error.
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    /// The directory entry error.
    #[fail(display = "entry: {}", _0)]
    Entry(io::Error),
}

impl Directory {
    const FILE_PATHS_INITIAL_CAPACITY: usize = 16;

    ///
    /// If the `path` does not end with the directory subpath, appends the subpath to the `path`.
    ///
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH) {
            path.push(PathBuf::from(
                zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH,
            ));
        }
        path
    }

    ///
    /// Returns the list of the Zinc binary files in the directory.
    ///
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH) {
            path.push(PathBuf::from(
                zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH,
            ));
        }

        let directory = fs::read_dir(path).map_err(Error::Reading)?;
        let mut file_paths = Vec::with_capacity(Self::FILE_PATHS_INITIAL_CAPACITY);
        for directory_entry in directory.into_iter() {
            let directory_entry = directory_entry.map_err(Error::Entry)?;
            let path = directory_entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension() {
                if extension == zinc_const::extensions::BYTECODE {
                    file_paths.push(path);
                }
            }
        }

        Ok(file_paths)
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
        if path.is_dir() && !path.ends_with(zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH) {
            path.push(PathBuf::from(
                zinc_const::zargo::TEST_BUILD_DIRECTORY_SUBPATH,
            ));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(Error::Removing)?;
        }

        Ok(())
    }
}
