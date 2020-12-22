//!
//! The source code directory.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Context;

use crate::error::Error as CompilerError;
use crate::generator::module::Module;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::scope::Scope;
use crate::source::error::Error;
use crate::source::file::File;
use crate::source::Source;

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
    pub modules: HashMap<String, Source>,
}

impl Directory {
    ///
    /// Initializes an application directory from string data.
    ///
    pub fn try_from_string(
        directory: zinc_project::Directory,
        is_entry: bool,
    ) -> anyhow::Result<Self> {
        let path = PathBuf::from(directory.path);
        let name = directory.name;

        let mut entry = None;
        let mut modules = HashMap::new();

        for (name, module) in directory.modules.into_iter() {
            match module {
                zinc_project::Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(Error::ModuleEntryInRoot)
                            .with_context(|| path.to_string_lossy().to_string());
                    }

                    if !is_entry && file.is_project_entry() {
                        return Err(Error::ProjectEntryBeyondRoot)
                            .with_context(|| path.to_string_lossy().to_string());
                    }

                    let file = File::try_from_string(file)
                        .with_context(|| path.to_string_lossy().to_string())?;

                    if file.is_entry() {
                        entry = Some(file);
                    } else {
                        modules.insert(name, Source::File(file));
                    }
                }
                zinc_project::Source::Directory(directory) => {
                    let directory = Self::try_from_string(directory, false)
                        .with_context(|| path.to_string_lossy().to_string())?;

                    modules.insert(name, Source::Directory(directory));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path,
                name,
                entry,
                modules,
            }),
            None if is_entry => {
                Err(Error::ProjectEntryNotFound).with_context(|| path.to_string_lossy().to_string())
            }
            None => {
                Err(Error::ModuleEntryNotFound).with_context(|| path.to_string_lossy().to_string())
            }
        }
    }

    ///
    /// Initializes an application module from a hard disk directory.
    ///
    pub fn try_from_path(path: &PathBuf, is_entry: bool) -> anyhow::Result<Self> {
        let directory = fs::read_dir(path).with_context(|| path.to_string_lossy().to_string())?;

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .with_context(|| path.to_string_lossy().to_string())?
            .to_string_lossy()
            .to_string();

        let mut entry = None;
        let mut modules = HashMap::new();

        for directory_entry in directory.into_iter() {
            let directory_entry =
                directory_entry.with_context(|| path.to_string_lossy().to_string())?;
            let path = directory_entry.path();
            let module = Source::try_from_path(&path)?;
            let name = module.name().to_owned();

            match module {
                Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(Error::ModuleEntryInRoot)
                            .with_context(|| path.to_string_lossy().to_string());
                    }

                    if !is_entry && file.is_project_entry() {
                        return Err(Error::ProjectEntryBeyondRoot)
                            .with_context(|| path.to_string_lossy().to_string());
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
                modules,
            }),
            None if is_entry => {
                Err(Error::ProjectEntryNotFound).with_context(|| path.to_string_lossy().to_string())
            }
            None => {
                Err(Error::ModuleEntryNotFound).with_context(|| path.to_string_lossy().to_string())
            }
        }
    }

    ///
    /// Runs the semantic analyzer on the syntax tree and returns the module scope.
    ///
    /// Used mostly for analyzing dependencies before attaching them to the main scope tree.
    ///
    pub fn modularize(
        self,
        project: zinc_project::ManifestProject,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> anyhow::Result<Rc<RefCell<Scope>>> {
        Ok(
            EntryAnalyzer::define(Source::Directory(self), project, dependencies, true)
                .map_err(CompilerError::Semantic)
                .map_err(|error| error.format())
                .map_err(Error::Compiling)?,
        )
    }

    ///
    /// Gets all the intermediate representation scattered around the application scope tree and
    /// writes it to the bytecode.
    ///
    pub fn compile(
        self,
        manifest: zinc_project::Manifest,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> anyhow::Result<Rc<RefCell<ZincVMState>>> {
        let scope = EntryAnalyzer::define(
            Source::Directory(self),
            manifest.project.clone(),
            dependencies,
            false,
        )
        .map_err(CompilerError::Semantic)
        .map_err(|error| error.format())
        .map_err(Error::Compiling)?;

        let state = ZincVMState::new(manifest).wrap();
        Module::new(scope.borrow().get_intermediate()).write_to_zinc_vm(state.clone());

        Ok(state)
    }

    ///
    /// Initialized a test module directory.
    ///
    pub fn test(
        code: &str,
        path: PathBuf,
        modules: HashMap<String, Source>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            path: path.clone(),
            name: "test".to_owned(),
            entry: File::test(code, path)?,
            modules,
        })
    }
}
