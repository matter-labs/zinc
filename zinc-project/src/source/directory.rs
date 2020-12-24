//!
//! The source code directory string representation.
//!

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::source::Source;

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
    pub fn try_from_path(path: &PathBuf, prefix: &PathBuf, is_entry: bool) -> anyhow::Result<Self> {
        let directory = fs::read_dir(&path).with_context(|| path.to_string_lossy().to_string())?;

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .with_context(|| path.to_string_lossy().to_string())?
            .to_string_lossy()
            .to_string();

        let mut entry_exists = false;
        let mut modules = HashMap::new();

        for directory_entry in directory.into_iter() {
            let directory_entry =
                directory_entry.with_context(|| path.to_string_lossy().to_string())?;
            let path = directory_entry.path();
            let module = Source::try_from_path(&path, prefix, false)
                .with_context(|| path.to_string_lossy().to_string())?;
            let name = module.name().to_owned();

            match module {
                Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(Error::ModuleEntryInRoot)
                            .with_context(|| path.to_string_lossy().to_string());
                    }

                    if !is_entry && file.is_project_entry() {
                        return Err(Error::ApplicationEntryBeyondRoot)
                            .with_context(|| path.to_string_lossy().to_string());
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

        if entry_exists {
            let path = path
                .strip_prefix(prefix)
                .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
                .to_path_buf()
                .to_string_lossy()
                .to_string();

            Ok(Self {
                name,
                path,
                modules,
            })
        } else if is_entry {
            Err(Error::ApplicationEntryNotFound).with_context(|| path.to_string_lossy().to_string())
        } else {
            Err(Error::ModuleEntryNotFound).with_context(|| path.to_string_lossy().to_string())
        }
    }

    ///
    /// Writes the directory with all inner elements to the disk.
    ///
    pub fn write_to(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut dir_path = path.to_owned();
        dir_path.push(&self.path);
        fs::create_dir_all(&dir_path).with_context(|| dir_path.to_string_lossy().to_string())?;

        for (_name, file) in self.modules.iter() {
            file.write_to(path)
                .with_context(|| path.to_string_lossy().to_string())?;
        }

        Ok(())
    }
}
