//!
//! The source code file string representation.
//!

pub mod error;

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::source::error::Error as SourceError;

use self::error::Error;

///
/// The Zinc virtual source code file, which consists of its name and source code string.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// The virtual file name without the extension.
    pub name: String,
    /// The virtual file subpath.
    pub path: String,
    /// The source code string data.
    pub code: String,
}

impl File {
    ///
    /// Initializes a virtual application module from a hard disk file.
    ///
    pub fn try_from_path(path: &PathBuf) -> Result<Self, SourceError> {
        let mut file = fs::File::open(&path)
            .map_err(Error::Opening)
            .map_err(SourceError::File)?;

        let size = file
            .metadata()
            .map_err(Error::Metadata)
            .map_err(SourceError::File)?
            .len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code)
            .map_err(Error::Reading)
            .map_err(SourceError::File)?;

        let source_file_extension = path
            .extension()
            .ok_or(Error::ExtensionNotFound)
            .map_err(SourceError::File)?;
        if source_file_extension != zinc_const::extension::SOURCE {
            return Err(SourceError::File(Error::ExtensionInvalid(
                source_file_extension.to_owned(),
            )));
        }

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(SourceError::File)?
            .to_string_lossy()
            .to_string();

        Ok(Self {
            name,
            path: path.to_string_lossy().to_string(),
            code,
        })
    }

    ///
    /// Checks whether the file is the entry point.
    ///
    pub fn is_entry(&self) -> bool {
        self.is_application_entry() || self.is_module_entry()
    }

    ///
    /// Checks whether the file is the application entry point.
    ///
    pub fn is_application_entry(&self) -> bool {
        self.path.as_str() == zinc_const::file_name::APPLICATION_ENTRY
    }

    ///
    /// Checks whether the file is the module entry point.
    ///
    pub fn is_module_entry(&self) -> bool {
        self.path.as_str() == zinc_const::file_name::MODULE_ENTRY
    }
}
