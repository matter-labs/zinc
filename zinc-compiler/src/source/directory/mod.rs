//!
//! The source code directory.
//!

pub mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::module::Module;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::source::error::Error as SourceError;
use crate::source::file::File;
use crate::source::Source;

use self::error::Error;

///
/// The Zinc source code directory, which consists of its path, root module (usually `mod.zn`),
/// and dependency modules.
///
#[derive(Debug, Clone)]
pub struct Directory {
    /// The directory name.
    pub name: String,
    /// The full directory path.
    pub path: PathBuf,
    /// The directory entry file, that is, a module, library, or application entry.
    pub entry: File,
    /// The module dependencies.
    pub dependencies: HashMap<String, Source>,
}

impl Directory {
    ///
    /// Initializes an application directory from string data.
    ///
    pub fn try_from_string(
        directory: zinc_data::Directory,
        is_entry: bool,
    ) -> Result<Self, SourceError> {
        let path = PathBuf::from(directory.path);

        let mut entry = None;
        let mut dependencies = HashMap::new();

        for (name, module) in directory.modules.into_iter() {
            match module {
                zinc_data::Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(SourceError::Directory(Error::ModuleEntryInRoot));
                    }

                    if !is_entry && file.is_application_entry() {
                        return Err(SourceError::Directory(Error::ApplicationEntryBeyondRoot));
                    }

                    let file = File::try_from_string(file)?;

                    if file.is_entry() {
                        entry = Some(file);
                    } else {
                        dependencies.insert(name, Source::File(file));
                    }
                }
                zinc_data::Source::Directory(directory) => {
                    let directory = Self::try_from_string(directory, false)?;

                    dependencies.insert(name, Source::Directory(directory));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path,
                name: directory.name,
                entry,
                dependencies,
            }),
            None if is_entry => Err(SourceError::Directory(Error::ApplicationEntryNotFound)),
            None => Err(SourceError::Directory(Error::ModuleEntryNotFound)),
        }
    }

    ///
    /// Initializes an application module from a hard disk directory.
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

        let mut entry = None;
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
                        entry = Some(file);
                    } else {
                        modules.insert(name, Source::File(file));
                    }
                }
                Source::Directory(directory) => {
                    modules.insert(name, Source::Directory(directory));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path: path.to_owned(),
                name,
                entry,
                dependencies: modules,
            }),
            None if is_entry => Err(SourceError::Directory(Error::ApplicationEntryNotFound)),
            None => Err(SourceError::Directory(Error::ModuleEntryNotFound)),
        }
    }

    ///
    /// Gets all the intermediate representation scattered around the application scope tree and
    /// writes it to the bytecode.
    ///
    pub fn compile(self, name: String) -> Result<Rc<RefCell<State>>, SourceError> {
        let scope = EntryAnalyzer::define(Source::Directory(self))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        let state = State::new(name).wrap();
        Module::new(scope.borrow().get_intermediate()).write_all(state.clone());

        Ok(state)
    }

    ///
    /// Initialized a test module directory.
    ///
    pub fn test(
        code: &str,
        path: PathBuf,
        dependencies: HashMap<String, Source>,
    ) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.clone(),
            name: "test".to_owned(),
            entry: File::test(code, path)?,
            dependencies,
        })
    }
}
