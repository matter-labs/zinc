//!
//! The project `build/test` directory.
//!

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

pub struct Directory {}

static DIRECTORY_NAME_DEFAULT: &str = "build/test/";

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "entry: {}", _0)]
    Entry(io::Error),
}

impl Directory {
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }
        path
    }

    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }

        let directory = fs::read_dir(path).map_err(Error::Reading)?;
        let mut file_paths = vec![];
        for directory_entry in directory.into_iter() {
            let directory_entry = directory_entry.map_err(Error::Entry)?;
            let path = directory_entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension() {
                if extension == "znb" {
                    file_paths.push(path);
                }
            }
        }

        Ok(file_paths)
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
