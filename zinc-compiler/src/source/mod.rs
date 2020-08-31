//!
//! The source code.
//!

pub mod directory;
pub mod error;
pub mod file;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::generator::state::State;

use self::directory::Directory;
use self::error::Error;
use self::file::File;

///
/// The file system source code representation.
///
#[derive(Debug, Clone)]
pub enum Source {
    /// The file system data file.
    File(File),
    /// The file system directory.
    Directory(Directory),
}

impl Source {
    ///
    /// Initializes an application module from string data.
    ///
    pub fn try_from_string(source: zinc_source::Source, is_entry: bool) -> Result<Self, Error> {
        Ok(match source {
            zinc_source::Source::File(inner) => File::try_from_string(inner).map(Source::File)?,
            zinc_source::Source::Directory(inner) => {
                Directory::try_from_string(inner, is_entry).map(Source::Directory)?
            }
        })
    }

    ///
    /// Initializes the entry application module representation from the file system.
    ///
    pub fn try_from_entry(path: &PathBuf) -> Result<Self, Error> {
        let file_type = fs::metadata(path).map_err(Error::FileMetadata)?.file_type();

        if file_type.is_dir() {
            return Directory::try_from_path(path, true).map(Self::Directory);
        }

        if file_type.is_file() {
            return File::try_from_path(path).map(Self::File);
        }

        Err(Error::FileTypeUnknown)
    }

    ///
    /// Initializes an application module representation from the file system.
    ///
    pub fn try_from_path(path: &PathBuf) -> Result<Self, Error> {
        let file_type = fs::metadata(path).map_err(Error::FileMetadata)?.file_type();

        if file_type.is_dir() {
            return Directory::try_from_path(path, false).map(Self::Directory);
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
    pub fn compile(self, name: String) -> Result<Rc<RefCell<State>>, Error> {
        match self {
            Self::File(inner) => inner.compile(name),
            Self::Directory(inner) => inner.compile(name),
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
    /// Initializes a test module.
    ///
    pub fn test(
        code: &str,
        path: PathBuf,
        file_index: usize,
        dependencies: HashMap<String, Source>,
    ) -> Self {
        if dependencies.is_empty() {
            File::test(code, path, file_index).map(Self::File)
        } else {
            Directory::test(code, path, file_index, dependencies).map(Self::Directory)
        }
        .expect(zinc_const::panic::TEST_DATA_VALID)
    }
}
