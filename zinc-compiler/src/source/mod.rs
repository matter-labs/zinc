//!
//! The source code.
//!

pub mod error;
pub mod module;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use crate::error::Error as CompilerError;

use self::error::Error;
use self::module::file::File;
use self::module::Module;

///
/// The Zinc project source code, which consists of some modules and the entry point.
///
#[derive(Debug, Clone)]
pub struct Source {
    pub path: PathBuf,
    pub entry: File,
    pub modules: HashMap<String, Module>,
}

impl Source {
    pub fn test(input: &str, dependencies: HashMap<String, Module>) -> Result<Self, CompilerError> {
        Ok(Self {
            path: PathBuf::from("test.zn"),
            entry: File::test(input)?,
            modules: dependencies,
        })
    }
}

impl TryFrom<&PathBuf> for Source {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let module = Module::try_from(path).map_err(|error| Error::RootModule {
            path: path.to_owned().into_os_string(),
            inner: error,
        })?;

        match module {
            Module::File(file) => {
                if file.is_application_entry() {
                    Ok(Self {
                        path: path.to_owned(),
                        entry: file,
                        modules: HashMap::new(),
                    })
                } else {
                    Err(Error::EntrySourceFileNotFound)
                }
            }
            Module::Directory(directory) => {
                if !directory.entry.is_application_entry() {
                    return Err(Error::EntrySourceFileNotFound);
                }

                Ok(Self {
                    path: directory.path,
                    entry: directory.entry,
                    modules: directory.modules,
                })
            }
        }
    }
}
