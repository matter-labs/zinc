//!
//! The source code module.
//!

pub mod directory;
pub mod error;
pub mod file;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;

use self::directory::Directory;
use self::error::Error;
use self::file::File;

#[derive(Debug, Clone)]
pub enum Source {
    File(File),
    Directory(Directory),
}

impl Source {
    ///
    /// Initializes an application module representation.
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
    /// Gets all the intermediate represenation scattered around the application scope tree and
    /// writes it to the bytecode.
    ///
    pub fn compile(self) -> Result<Rc<RefCell<Bytecode>>, Error> {
        match self {
            Self::File(inner) => inner.compile(),
            Self::Directory(inner) => inner.compile(),
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

    ///
    /// Checks whether the module is the application entry point file.
    ///
    pub fn is_application_entry(&self) -> bool {
        match self {
            Self::File(file) => file.is_application_entry(),
            Self::Directory(_directory) => false,
        }
    }

    ///
    /// Checks whether the module is the module entry point file.
    ///
    pub fn is_module_entry(&self) -> bool {
        match self {
            Self::File(file) => file.is_module_entry(),
            Self::Directory(_directory) => false,
        }
    }

    ///
    /// Initialized a test module.
    ///
    pub fn test(input: &str, dependencies: HashMap<String, Source>) -> Self {
        if dependencies.is_empty() {
            File::test(input).map(Self::File)
        } else {
            Directory::test(input, dependencies).map(Self::Directory)
        }
        .expect(crate::panic::TEST_DATA)
    }
}
