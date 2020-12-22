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

use anyhow::Context;

use crate::generator::zinc_vm::State as ZincVMState;
use crate::semantic::scope::Scope;
use crate::source::error::Error;

use self::directory::Directory;
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
    pub fn try_from_string(source: zinc_project::Source, is_entry: bool) -> anyhow::Result<Self> {
        match source {
            zinc_project::Source::File(inner) => File::try_from_string(inner).map(Self::File),
            zinc_project::Source::Directory(inner) => {
                Directory::try_from_string(inner, is_entry).map(Self::Directory)
            }
        }
    }

    ///
    /// Initializes the entry application module representation from the file system.
    ///
    pub fn try_from_entry(path: &PathBuf) -> anyhow::Result<Self> {
        let file_type = fs::metadata(path)
            .with_context(|| path.to_string_lossy().to_string())?
            .file_type();

        if file_type.is_dir() {
            return Directory::try_from_path(path, true).map(Self::Directory);
        }

        if file_type.is_file() {
            return File::try_from_path(path).map(Self::File);
        }

        Err(Error::FileTypeUnknown).with_context(|| path.to_string_lossy().to_string())
    }

    ///
    /// Initializes an application module representation from the file system.
    ///
    pub fn try_from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let file_type = fs::metadata(path)
            .with_context(|| path.to_string_lossy().to_string())?
            .file_type();

        if file_type.is_dir() {
            return Directory::try_from_path(path, false).map(Self::Directory);
        }

        if file_type.is_file() {
            return File::try_from_path(path).map(Self::File);
        }

        Err(Error::FileTypeUnknown).with_context(|| path.to_string_lossy().to_string())
    }

    ///
    /// Runs the semantic analyzer on the syntax tree and returns the module scope.
    ///
    pub fn modularize(
        self,
        project: zinc_project::ManifestProject,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> anyhow::Result<Rc<RefCell<Scope>>> {
        match self {
            Self::File(inner) => inner.modularize(project, dependencies),
            Self::Directory(inner) => inner.modularize(project, dependencies),
        }
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
        match self {
            Self::File(inner) => inner.compile(manifest, dependencies),
            Self::Directory(inner) => inner.compile(manifest, dependencies),
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
            Self::File(file) => file.is_project_entry(),
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
        modules: HashMap<String, Source>,
    ) -> anyhow::Result<Self> {
        if modules.is_empty() {
            File::test(code, path).map(Self::File)
        } else {
            Directory::test(code, path, modules).map(Self::Directory)
        }
    }
}
