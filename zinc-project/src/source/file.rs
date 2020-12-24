//!
//! The source code file string representation.
//!

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use anyhow::Context;

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
    pub fn try_from_path(path: &PathBuf, prefix: &PathBuf) -> anyhow::Result<Self> {
        let mut file = fs::File::open(path).with_context(|| path.to_string_lossy().to_string())?;

        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code)
            .with_context(|| path.to_string_lossy().to_string())?;

        let source_file_extension = path
            .extension()
            .ok_or(Error::ExtensionNotFound)
            .with_context(|| path.to_string_lossy().to_string())?;
        if source_file_extension != zinc_const::extension::SOURCE {
            return Err(Error::ExtensionInvalid(source_file_extension.to_owned()))
                .with_context(|| path.to_string_lossy().to_string());
        }

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .with_context(|| path.to_string_lossy().to_string())?
            .to_string_lossy()
            .to_string();

        let path = path
            .strip_prefix(prefix)
            .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .to_path_buf()
            .to_string_lossy()
            .to_string();

        Ok(Self { name, path, code })
    }

    ///
    /// Writes the directory with all inner elements to the disk.
    ///
    pub fn write_to(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut path = path.to_owned();
        path.push(&self.path);

        let mut file =
            fs::File::create(&path).with_context(|| path.to_string_lossy().to_string())?;
        file.write_all(self.code.as_bytes())
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(())
    }

    ///
    /// Checks whether the file is the entry point.
    ///
    pub fn is_entry(&self) -> bool {
        self.is_project_entry() || self.is_module_entry()
    }

    ///
    /// Checks whether the file is the project entry point.
    ///
    pub fn is_project_entry(&self) -> bool {
        self.name.as_str() == zinc_const::file_name::APPLICATION_ENTRY
            || self.name.as_str() == zinc_const::file_name::LIBRARY_ENTRY
    }

    ///
    /// Checks whether the file is the module entry point.
    ///
    pub fn is_module_entry(&self) -> bool {
        self.name.as_str() == zinc_const::file_name::MODULE_ENTRY
    }
}
