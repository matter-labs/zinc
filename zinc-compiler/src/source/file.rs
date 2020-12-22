//!
//! The source code file.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Context;

use zinc_lexical::FILE_INDEX;
use zinc_syntax::Module as SyntaxModule;
use zinc_syntax::Parser;

use crate::error::Error as CompilerError;
use crate::generator::module::Module;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::scope::Scope;
use crate::source::error::Error;
use crate::source::Source;

///
/// The Zinc source code file, which consists of its path and parsed syntax tree.
///
#[derive(Debug, Clone)]
pub struct File {
    /// The file name without the extension.
    pub name: String,
    /// The full file path.
    pub path: PathBuf,
    /// The source code syntax tree.
    pub tree: SyntaxModule,
}

impl File {
    ///
    /// Initializes an application module from a string.
    ///
    /// `path` is used to set the virtual module path within a project.
    ///
    pub fn try_from_string(file: zinc_project::File) -> anyhow::Result<Self> {
        let path = PathBuf::from(file.path);

        let next_file_id = FILE_INDEX.next(&path, file.code);
        let tree = Parser::default()
            .parse(
                FILE_INDEX
                    .inner
                    .read()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .get(&next_file_id)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .code
                    .as_str(),
                next_file_id,
            )
            .map_err(CompilerError::from)
            .map_err(|error| error.format())
            .map_err(Error::Compiling)?;

        Ok(Self {
            path,
            name: file.name,
            tree,
        })
    }

    ///
    /// Initializes an application module from a hard disk file.
    ///
    pub fn try_from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let mut file = fs::File::open(&path).with_context(|| path.to_string_lossy().to_string())?;

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

        let next_file_id = FILE_INDEX.next(path, code);
        let tree = Parser::default()
            .parse(
                FILE_INDEX
                    .inner
                    .read()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .get(&next_file_id)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .code
                    .as_str(),
                next_file_id,
            )
            .map_err(CompilerError::from)
            .map_err(|error| error.format())
            .map_err(Error::Compiling)?;

        Ok(Self {
            path: path.to_owned(),
            name,
            tree,
        })
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
            EntryAnalyzer::define(Source::File(self), project, dependencies, true)
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
            Source::File(self),
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

    ///
    /// Initializes a test module file.
    ///
    pub fn test(code: &str, path: PathBuf) -> anyhow::Result<Self> {
        let next_file_id = FILE_INDEX.peek();

        let tree = Parser::default()
            .parse(code, next_file_id)
            .map_err(CompilerError::from)
            .map_err(|error| error.format())
            .map_err(Error::Compiling)?;

        FILE_INDEX.next(&path, code.to_owned());

        Ok(Self {
            path,
            name: format!("test_#{}", next_file_id),
            tree,
        })
    }
}
