//!
//! The project `src` directory.
//!

pub mod circuit;
pub mod contract;

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

pub struct Directory {}

pub(self) static DIRECTORY_NAME_DEFAULT: &str = "src/";
pub(self) static SOURCE_FILE_EXTENSION_DEFAULT: &str = "zn";

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
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
}
