//!
//! The source code directory.
//!

pub mod error;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use crate::error::Error as CompilerError;
use crate::source::module::error::Error as ModuleError;
use crate::source::module::file::File;
use crate::source::module::Module;

use self::error::Error;

///
/// The Zinc source code directory, which consists of its path, root module, and dependency modules.
///
#[derive(Debug, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub name: String,
    pub entry: File,
    pub modules: HashMap<String, Module>,
}

impl Directory {
    pub fn test(input: &str, dependencies: HashMap<String, Module>) -> Result<Self, CompilerError> {
        Ok(Self {
            path: PathBuf::from("test.zn"),
            name: "test".to_owned(),
            entry: File::test(input)?,
            modules: dependencies,
        })
    }
}

impl TryFrom<&PathBuf> for Directory {
    type Error = ModuleError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let directory = fs::read_dir(path)
            .map_err(Error::Reading)
            .map_err(ModuleError::Directory)?;

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(ModuleError::Directory)?
            .to_string_lossy()
            .to_string();

        let mut entry = None;
        let mut modules = HashMap::new();

        for directory_entry in directory.into_iter() {
            let directory_entry = directory_entry
                .map_err(Error::DirectoryEntry)
                .map_err(ModuleError::Directory)?;
            let path = directory_entry.path();
            let module = Module::try_from(&path)?;
            let name = module.name().to_owned();

            if let Module::File(file) = module {
                if file.is_application_entry() || file.is_module_entry() {
                    entry = Some(file);
                } else {
                    modules.insert(name, Module::File(file));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path: path.to_owned(),
                name,
                entry,
                modules,
            }),
            None => Err(ModuleError::Directory(Error::EntrySourceFileNotFound)),
        }
    }
}
