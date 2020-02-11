//!
//! The Zinc tester directory.
//!

use std::ffi::OsString;
use std::fs;
use std::fs::FileType;
use std::io;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestDirectory {
    pub file_paths: Vec<PathBuf>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "file entry getting: {}", _0)]
    GettingFileEntry(io::Error),
    #[fail(display = "file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),
    #[fail(display = "invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),
    #[fail(display = "file {:?} extension getting", _0)]
    GettingFileExtension(OsString),
    #[fail(display = "invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(OsString, OsString),
}

static TEST_FILE_EXTENSION_DEFAULT: &str = "zn";

impl TestDirectory {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let directory = fs::read_dir(path).map_err(Error::Reading)?;
        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(Error::GettingFileEntry)?;
            let file_path = file_entry.path();

            let file_type = file_entry
                .file_type()
                .map_err(|error| Error::GettingFileType(file_path.as_os_str().to_owned(), error))?;
            if !file_type.is_file() {
                return Err(Error::InvalidFileType(
                    file_path.as_os_str().to_owned(),
                    file_type,
                ));
            }

            let file_extension = file_path
                .extension()
                .ok_or_else(|| Error::GettingFileExtension(file_path.as_os_str().to_owned()))?;
            if file_extension != TEST_FILE_EXTENSION_DEFAULT {
                return Err(Error::InvalidFileExtension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(file_path);
        }
        Ok(Self { file_paths })
    }
}
