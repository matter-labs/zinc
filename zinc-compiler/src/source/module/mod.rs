//!
//! The source code module.
//!

pub mod directory;
pub mod error;
pub mod file;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use crate::error::Error as CompilerError;

use self::directory::Directory;
use self::error::Error;
use self::file::File;

#[derive(Debug, Clone)]
pub enum Module {
    File(File),
    Directory(Directory),
}

impl Module {
    pub fn test(input: &str, dependencies: HashMap<String, Module>) -> Result<Self, CompilerError> {
        if dependencies.is_empty() {
            File::test(input).map(Self::File)
        } else {
            Directory::test(input, dependencies).map(Self::Directory)
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::File(inner) => inner.name.as_str(),
            Self::Directory(inner) => inner.name.as_str(),
        }
    }

    pub fn is_application_entry(&self) -> bool {
        match self {
            Self::File(file) => file.is_application_entry(),
            Self::Directory(_directory) => false,
        }
    }

    pub fn is_module_entry(&self) -> bool {
        match self {
            Self::File(file) => file.is_module_entry(),
            Self::Directory(_directory) => false,
        }
    }
}

impl TryFrom<&PathBuf> for Module {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let file_type = fs::metadata(path).map_err(Error::FileMetadata)?.file_type();

        if file_type.is_dir() {
            return Directory::try_from(path).map(Self::Directory);
        }

        if file_type.is_file() {
            return File::try_from(path).map(Self::File);
        }

        Err(Error::FileTypeUnknown)
    }
}
