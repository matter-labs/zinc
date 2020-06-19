//!
//! The project `build` directory.
//!

pub mod test;

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

pub struct Directory {}

static DIRECTORY_NAME_DEFAULT: &str = "build/";

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
}

impl Directory {
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }
        path
    }

    pub fn create(path: &PathBuf) -> Result<(), Error> {
        fs::create_dir_all(&Self::path(path)).map_err(Error::Creating)
    }

    pub fn remove(path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(Error::Removing)?;
        }

        Ok(())
    }
}
