//!
//! The source code directory string representation.
//!

pub mod error;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::source::error::Error as SourceError;
use crate::source::Source;

use self::error::Error;

///
/// The Zinc virtual source code directory, which consists of its name and virtual files.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    /// The virtual directory name.
    pub name: String,
    /// The virtual directory subpath.
    pub path: String,
    /// The virtual dependency files.
    pub modules: HashMap<String, Source>,
}

impl Directory {
    ///
    /// Initializes a virtual application module from a hard disk directory.
    ///
    pub fn try_from_path(path: &PathBuf, is_entry: bool) -> Result<Self, SourceError> {
        let directory = fs::read_dir(path)
            .map_err(Error::Reading)
            .map_err(SourceError::Directory)?;

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(SourceError::Directory)?
            .to_string_lossy()
            .to_string();

        let mut entry_exists = false;
        let mut modules = HashMap::new();

        for directory_entry in directory.into_iter() {
            let directory_entry = directory_entry
                .map_err(Error::DirectoryEntry)
                .map_err(SourceError::Directory)?;
            let path = directory_entry.path();
            let module = Source::try_from_path(&path)?;
            let name = module.name().to_owned();

            match module {
                Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(SourceError::Directory(Error::ModuleEntryInRoot));
                    }

                    if !is_entry && file.is_application_entry() {
                        return Err(SourceError::Directory(Error::ApplicationEntryBeyondRoot));
                    }

                    if file.is_entry() {
                        entry_exists = true;
                    }

                    modules.insert(name, Source::File(file));
                }
                Source::Directory(directory) => {
                    modules.insert(name, Source::Directory(directory));
                }
            }
        }

        if is_entry {
            if entry_exists {
                Ok(Self {
                    name,
                    path: path.to_string_lossy().to_string(),
                    modules,
                })
            } else {
                Err(SourceError::Directory(Error::ApplicationEntryNotFound))
            }
        } else {
            Err(SourceError::Directory(Error::ModuleEntryNotFound))
        }
    }
}
